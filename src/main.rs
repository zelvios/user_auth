mod db;
mod handlers;
mod models;
mod schema;
mod services;
mod utils;

use crate::handlers::users::{view_own_user, view_user_table, view_users};
use axum::{
    routing::{get, post}, Extension,
    Router,
};
use handlers::{
    auth::login,
    permissions::view_permissions_table,
    users::{create_user},
    roles::view_role_table,
};
use std::{env, sync::Arc};
use crate::handlers::roles::view_roles;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = Arc::new(db::establish_connection_pool());
    let jwt_secret = Arc::new(env::var("JWT_SECRET").expect("JWT_SECRET must be set"));

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user).get(view_users))
        .route("/users/profile", get(view_own_user))
        // Need Permissions:
        .route("/dev/users", get(view_user_table))
        .route("/dev/roles", get(view_role_table))
        .route("/roles", get(view_roles))
        .route("/permissions", get(view_permissions_table))
        .route("/auth", post(login))
        .layer(Extension(pool))
        .layer(Extension(jwt_secret));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hi! This API is used for the project 'user_auth'"
}
