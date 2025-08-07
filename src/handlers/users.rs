use crate::models::{UserTableView, UserView};
use crate::services::jwt::extract_user_from_jwt;
use crate::{
    db::Pool,
    models::{NewUser, NewUserInput, User},
    schema::users::dsl::*,
    services::permissions::user_has_permission,
    utils::{error::internal_error, hash::hash_password},
};
use axum::{
    http::{HeaderMap, StatusCode}, Extension,
    Json,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

/// Create a new user.
///
/// **Authentication:** No authentication required.
///
/// Accepts a JSON payload based on the `NewUserInput` struct containing minimal
/// user information to create a new user in the database.
/// ___
/// # Returns
/// - `201 Created` with the email on success.
/// - `500 INTERNAL_SERVER_ERROR` on database error.
/// ___
/// ## `NewUserInput` JSON Payload Example
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "password123",
///   "first_name": "John",
///   "last_name": "Doe"
/// }
/// ```
pub async fn create_user(
    Extension(pool): Extension<Arc<Pool>>,
    Json(payload): Json<NewUserInput>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| internal_error("DB Pool error", e))?;

    let hashed_password = hash_password(&payload.password)
        .map_err(|e| internal_error("Password hashing failed", e))?;

    let new_user = NewUser {
        email: payload.email.to_lowercase(),
        username: payload.username,
        password_hash: hashed_password,
        first_name: payload.first_name,
        last_name: payload.last_name,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut conn)
        .map_err(|e| internal_error("DB insert error", e))?;

    Ok((StatusCode::CREATED, Json(new_user.email)))
}

/// Returns a list of all `users` from the database table.
///
/// **Authentication:** `can_see_user_table`
///
/// Extracts user info from JWT in headers and verifies access.
/// ___
/// # Returns
/// - `200 OK` with JSON list of **users** on success.
/// - `403 FORBIDDEN` if user lacks permissions.
/// - `500 INTERNAL_SERVER_ERROR` on database error.
pub async fn view_user_table(
    Extension(pool): Extension<Arc<Pool>>,
    Extension(jwt_secret): Extension<Arc<String>>,
    headers: HeaderMap,
) -> Result<Json<Vec<UserTableView>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| internal_error("DB Pool error", e))?;

    let claims = extract_user_from_jwt(&jwt_secret, &headers).await?;

    const REQUIRED_PERMISSION: &str = "can_view_user_table";

    let allowed = user_has_permission(&claims, &mut conn, REQUIRED_PERMISSION).await?;
    if !allowed {
        return Err((
            StatusCode::FORBIDDEN,
            format!("Missing permission: {}", REQUIRED_PERMISSION),
        ));
    }

    let all_users = users
        .select(UserTableView::as_select())
        .load::<UserTableView>(&mut conn)
        .map_err(|e| internal_error("DB load error", e))?;

    Ok(Json(all_users))
}

/// Returns a list of all `users` from the database table without private credentials.
///
/// **Authentication:** No authentication required.
///
/// Extracts user info from JWT in headers and verifies access.
/// Returns the name of the roles instead of bitmask.
/// ___
/// # Returns
/// - `200 OK` with JSON list of **users** on success.
/// - `500 INTERNAL_SERVER_ERROR` on database error.
pub async fn view_users(
    Extension(pool): Extension<Arc<Pool>>,
) -> Result<Json<Vec<UserView>>, (StatusCode, String)> {
    use crate::schema::roles::dsl::{id as role_id, name as role_name, roles as roles_table};
    use crate::schema::users::dsl::*;

    let mut conn = pool.get().map_err(|e| internal_error("DB Pool error", e))?;

    let raw_users = users
        .select((email, username, first_name, last_name, roles, created_at))
        .load::<(String, String, String, String, i16, Option<DateTime<Utc>>)>(&mut conn)
        .map_err(|e| internal_error("DB load error", e))?;

    let all_roles = roles_table
        .select((role_id, role_name))
        .load::<(i32, String)>(&mut conn)
        .map_err(|e| internal_error("DB load roles error", e))?;

    let role_map: std::collections::HashMap<i32, String> = all_roles.into_iter().collect();

    let result: Vec<UserView> = raw_users
        .into_iter()
        .map(
            |(
                email_val,
                username_val,
                first_name_val,
                last_name_val,
                roles_bits,
                created_at_val,
            )| {
                let resolved_roles = (0..16)
                    .filter_map(|bit| {
                        if roles_bits & (1 << bit) != 0 {
                            role_map.get(&(bit)).cloned()
                        } else {
                            None
                        }
                    })
                    .collect();

                UserView {
                    email: email_val,
                    username: username_val,
                    first_name: first_name_val,
                    last_name: last_name_val,
                    roles: resolved_roles,
                    created_at: created_at_val
                        .map(|dt| dt.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "Unknown".to_string()),
                }
            },
        )
        .collect();

    Ok(Json(result))
}

/// Returns information about own `user` from the database table.
///
/// **Authentication:** `can_view_own_profile`
///
/// Extracts user info from JWT in headers and verifies access.
/// ___
/// # Returns
/// - `201 Created` with the users information serialized as JSON on success.
/// - `401 UNAUTHORIZED` if user id in JWT claims is invalid.
/// - `403 FORBIDDEN` if user lacks permissions.
/// - `500 INTERNAL_SERVER_ERROR` on database error.
pub async fn view_own_user(
    Extension(pool): Extension<Arc<Pool>>,
    Extension(jwt_secret): Extension<Arc<String>>,
    headers: HeaderMap,
) -> Result<Json<UserView>, (StatusCode, String)> {
    use crate::schema::roles::dsl::{id as role_id, name as role_name, roles as roles_table};
    use crate::schema::users::dsl::*;

    let mut conn = pool.get().map_err(|e| internal_error("DB Pool error", e))?;

    let claims = extract_user_from_jwt(&jwt_secret, &headers).await?;

    let temp_uuid = Uuid::parse_str(&claims.user_temp_id)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid UUID in token".into()))?;

    let (email_val, username_val, first_name_val, last_name_val, roles_bits, created_at_val) =
        users
            .filter(temp_id.eq(temp_uuid))
            .select((email, username, first_name, last_name, roles, created_at))
            .first::<(String, String, String, String, i16, Option<DateTime<Utc>>)>(&mut conn)
            .map_err(|e| internal_error("Failed to load user", e))?;

    let all_roles = roles_table
        .select((role_id, role_name))
        .load::<(i32, String)>(&mut conn)
        .map_err(|e| internal_error("Failed to load roles", e))?;

    let role_map: std::collections::HashMap<i32, String> = all_roles.into_iter().collect();

    let resolved_roles = (0..16)
        .filter_map(|bit| {
            if roles_bits & (1 << bit) != 0 {
                role_map.get(&bit).cloned()
            } else {
                None
            }
        })
        .collect();

    let user_view = UserView {
        email: email_val,
        username: username_val,
        first_name: first_name_val,
        last_name: last_name_val,
        roles: resolved_roles,
        created_at: created_at_val
            .map(|dt| dt.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
    };

    Ok(Json(user_view))
}

