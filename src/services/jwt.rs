use crate::models::User;
use axum::http::{HeaderMap, StatusCode};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub user_temp_id: String,
}

pub fn create_jwt(user: &User, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.email.clone(),
        exp: expiration as usize,
        user_temp_id: user.temp_id.to_string(),
    };

    let header = Header::new(Algorithm::HS256);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub async fn extract_user_from_jwt(
    jwt_secret: &str,
    headers: &HeaderMap,
) -> Result<Claims, (StatusCode, String)> {
    if let Some(auth_header_value) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header_value.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::new(Algorithm::HS256),
                )
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))?;

                return Ok(token_data.claims);
            }
        }
    }

    Err((
        StatusCode::UNAUTHORIZED,
        "Missing or malformed token".into(),
    ))
}
