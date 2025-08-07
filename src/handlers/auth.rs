use axum::{Extension, Json, http::StatusCode};
use std::sync::Arc;
use diesel::prelude::*;
use crate::{models::{Login, User}, schema::users::dsl::*, db::Pool, services::jwt::create_jwt, utils::{hash::verify_password, error::internal_error}};

/// Returns a list of all `permissions` from the database table.
///
/// **Authentication:** No authentication required.
///
/// Accepts a JSON payload based on the `Login` struct containing user credentials
/// to receive a JWT.
/// ___
/// # Returns
/// - `200 OK` with JWT token as JSON on successful login.
/// - `401 UNAUTHORIZED` if credentials are invalid.
/// - `403 FORBIDDEN` if the user account is inactive.
/// - `500 INTERNAL_SERVER_ERROR` on database or token generation error.
/// ---
/// ## `Login` JSON Payload Example
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "password123"
/// }
/// ```
pub async fn login(
    Extension(pool): Extension<Arc<Pool>>,
    Extension(jwt_secret): Extension<Arc<String>>,
    Json(payload): Json<Login>,
) -> Result<Json<String>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| internal_error("DB Pool error", e))?;

    let user = users
        .filter(email.eq(&payload.email.to_lowercase()))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|e| internal_error("DB query error", e))?;

    let user = match user {
        Some(user) => user,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into())),
    };

    if !user.is_active {
        return Err((StatusCode::FORBIDDEN, "Account is inactive".into()));
    }

    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| internal_error("Password verification failed", e))?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()));
    }

    let token =
        create_jwt(&user, &jwt_secret).map_err(|e| internal_error("JWT generation failed", e))?;

    Ok(Json(token))
}
