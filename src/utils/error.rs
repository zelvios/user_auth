use axum::http::StatusCode;

pub fn internal_error(msg: &str, e: impl std::fmt::Display) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{}: {}", msg, e))
}
