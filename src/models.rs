use super::schema::{users, roles, permissions};
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Identifiable, Selectable};
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub temp_id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub roles: i16,
    pub permissions: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub permission: i64,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub name: String,
    pub description: Option<String>,
    pub permission: i64,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = permissions)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = permissions)]
pub struct NewPermission {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewUserInput {
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = users)]
pub struct UserTableView {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub roles: i16,
    pub permissions: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = users)]
pub struct UserView {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<String>,
    pub created_at: String,
}
#[derive(Serialize)]
pub struct PermissionView {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = roles)]
pub struct RoleTableView {
    pub name: String,
    pub description: Option<String>,
    pub permission: Vec<PermissionView>,
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = roles)]
pub struct RoleView {
    pub name: String,
    pub description: Option<String>,
}