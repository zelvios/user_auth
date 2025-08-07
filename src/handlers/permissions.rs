use crate::services::jwt::extract_user_from_jwt;
use crate::services::permissions::user_has_permission;
use crate::{
    db::Pool, models::Permission, schema::permissions::dsl::*, utils::error::internal_error,
};
use axum::http::HeaderMap;
use axum::{http::StatusCode, Extension, Json};
use diesel::prelude::*;
use std::sync::Arc;

/// Returns a list of all `permissions` from the database table.
///
/// **Authentication:** `can_see_permission_table`
/// 
/// Extracts user info from JWT in headers and verifies access.
/// ___
/// # Returns
/// - `200 OK` with JSON list of **permissions** on success.
/// - `403 FORBIDDEN` if user lacks permissions.
/// - `500 INTERNAL_SERVER_ERROR` on database error.
pub async fn view_permissions_table(
    Extension(pool): Extension<Arc<Pool>>,
    Extension(jwt_secret): Extension<Arc<String>>,
    headers: HeaderMap,
) -> Result<Json<Vec<Permission>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| internal_error("DB Pool error", e))?;

    let claims = extract_user_from_jwt(&jwt_secret, &headers).await?;

    const REQUIRED_PERMISSION: &str = "can_view_permission_table";

    let allowed = user_has_permission(&claims, &mut conn, REQUIRED_PERMISSION).await?;
    if !allowed {
        return Err((
            StatusCode::FORBIDDEN,
            format!("Missing permission: {}", REQUIRED_PERMISSION),
        ));
    }

    let all_permissions = permissions
        .load::<Permission>(&mut conn)
        .map_err(|e| internal_error("DB load error", e))?;

    Ok(Json(all_permissions))
}
