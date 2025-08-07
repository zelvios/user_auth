use crate::models::{Permission, User};
use crate::services::jwt::Claims;
use crate::utils::error::internal_error;
use axum::http::StatusCode;
use diesel::{ExpressionMethods, PgConnection};
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn user_has_permission(
    claims: &Claims,
    conn: &mut PgConnection,
    permission_name: &str,
) -> Result<bool, (StatusCode, String)> {
    use crate::schema::permissions::dsl::{name as perm_name, permissions};
    use crate::schema::roles::dsl::{
        id as role_id, permission as role_permission, roles as roles_table,
    };
    use crate::schema::users::dsl::*;

    let temp_uuid = Uuid::parse_str(&claims.user_temp_id)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid UUID".into()))?;

    let user = users
        .filter(temp_id.eq(temp_uuid))
        .first::<User>(conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "User not found".into()))?;


    let perm = permissions
        .filter(perm_name.eq(permission_name))
        .first::<Permission>(conn)
        .optional()
        .map_err(|e| internal_error("Permission query failed", e))?;

    let perm = match perm {
        Some(p) => p,
        None => return Ok(false),
    };

    let perm_bit = 1i64 << (perm.id - 1);

    let all_roles = roles_table
        .select((role_id, role_permission))
        .load::<(i32, i64)>(conn)
        .map_err(|e| internal_error("Roles query failed", e))?;

    let user_roles_bitmask = user.roles as i32;

    let mut combined_role_perm: i64 = 0;

    for (rid, rperm) in all_roles {
        let role_bit = 1 << (rid - 1);

        if (user_roles_bitmask & role_bit) != 0 {
            combined_role_perm |= rperm;
        }
    }

    Ok((user.permissions & perm_bit) != 0 || (combined_role_perm & perm_bit) != 0)
}
