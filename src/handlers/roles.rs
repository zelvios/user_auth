use crate::models::{PermissionView, RoleTableView, RoleView};
use crate::schema::roles::dsl::roles;
use crate::services::jwt::extract_user_from_jwt;
use crate::services::permissions::user_has_permission;
use crate::{db::Pool, utils::error::internal_error};
use axum::{http::HeaderMap, http::StatusCode, Extension, Json};
use diesel::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn view_roles(
    Extension(pool): Extension<Arc<Pool>>,
) -> Result<Json<Vec<RoleView>>, (StatusCode, String)> {
    use crate::schema::roles::{description, name};

    let mut conn = pool.get().map_err(|e| internal_error("DB Pool error", e))?;

    let raw_roles = roles
        .select((name, description))
        .load::<(String, Option<String>)>(&mut conn)
        .map_err(|e| internal_error("DB load error", e))?;

    let result: Vec<RoleView> = raw_roles
        .into_iter()
        .map(|(name_val, description_val)| RoleView {
            name: name_val,
            description: description_val,
        })
        .collect();

    Ok(Json(result))
}

pub async fn view_role_table(
    Extension(pool): Extension<Arc<Pool>>,
    Extension(jwt_secret): Extension<Arc<String>>,
    headers: HeaderMap,
) -> Result<Json<Vec<RoleTableView>>, (StatusCode, String)> {
    use crate::schema::permissions::dsl::{
        description as permission_description, id as permission_id, name as permission_name,
        permissions as permission_table,
    };

    let mut conn = pool.get().map_err(|e| internal_error("DB Pool error", e))?;

    let claims = extract_user_from_jwt(&jwt_secret, &headers).await?;
    const REQUIRED_PERMISSION: &str = "can_view_role_table";

    let allowed = user_has_permission(&claims, &mut conn, REQUIRED_PERMISSION).await?;
    if !allowed {
        return Err((
            StatusCode::FORBIDDEN,
            format!("Missing permission: {}", REQUIRED_PERMISSION),
        ));
    }

    let raw_roles = roles
        .select((
            crate::schema::roles::dsl::name,
            crate::schema::roles::dsl::description,
            crate::schema::roles::dsl::permission,
        ))
        .load::<(String, Option<String>, i64)>(&mut conn)
        .map_err(|e| internal_error("DB load error", e))?;

    let all_permissions = permission_table
        .select((permission_id, permission_name, permission_description))
        .load::<(i32, String, Option<String>)>(&mut conn)
        .map_err(|e| internal_error("DB load roles error", e))?;

    let permission_map: HashMap<i32, (String, Option<String>)> = all_permissions
        .into_iter()
        .map(|(id, name, desc)| (id, (name, desc)))
        .collect();

    let result: Vec<RoleTableView> = raw_roles
        .into_iter()
        .map(|(name_val, description_val, permission_bits)| {
            let resolved_permissions: Vec<PermissionView> = (0..64)
                .filter_map(|bit| {
                    if permission_bits & (1 << bit) != 0 {
                        permission_map
                            .get(&(bit))
                            .map(|(perm_name, perm_desc)| PermissionView {
                                name: perm_name.clone(),
                                description: perm_desc.clone(),
                            })
                    } else {
                        None
                    }
                })
                .collect();

            RoleTableView {
                name: name_val,
                description: description_val,
                permission: resolved_permissions,
            }
        })
        .collect();

    Ok(Json(result))
}
