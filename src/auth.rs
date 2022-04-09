use actix_web::HttpRequest;
use chrono::prelude::*;
use jsonwebtoken::{self, decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::MyError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: String,
    exp: i64,
    iat: i64,
}

pub fn generate_access_token(user_id: Uuid) -> String {
    generate(
        user_id,
        chrono::Duration::seconds(30),
        "access_secret".to_string(),
    )
}

pub fn generate_refresh_token(user_id: Uuid) -> String {
    generate(
        user_id,
        chrono::Duration::days(7),
        "refresh_secret".to_string(),
    )
}

fn generate(user_id: Uuid, duration: chrono::Duration, secret: String) -> String {
    dbg!(duration.clone());
    let now = Utc::now();
    dbg!(now.clone());
    let exp = Utc::now() + duration;
    dbg!(exp.clone());

    let claims = Claims {
        user_id: user_id.to_string(),
        iat: now.timestamp(),
        exp: exp.timestamp(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(&secret.as_bytes()),
    )
    .unwrap_or_default()
}

pub fn get_auth_from_header(request: HttpRequest) -> Result<String, MyError> {
    match request
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
    {
        Some(auth_header) => {
            let auth_values = auth_header
                .to_str()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();
            match auth_values.first() {
                Some(schema) => {
                    // ===
                    match schema.trim().to_lowercase().as_str() {
                        "bearer" => Ok(auth_values[1].to_string()),
                        _ => Err(MyError::General {
                            desc: "Wrong Authentication scheme".to_string(),
                        }),
                    }
                }
                None => Err(MyError::General {
                    desc: "Empty `Authorization` schema".to_string(),
                }),
            }
        }
        // No `Authorization` header found
        None => Err(MyError::General {
            desc: "No `Authorization` header found".to_string(),
        }),
    }
}

/// Decode access_token and return user_id
pub fn decode_access_token(token: String) -> Result<String, MyError> {
    decode_token(token, "access_secret".to_string())
}

/// Decode refresh_token and return user_id
pub fn decode_refresh_token(token: String) -> Result<String, MyError> {
    decode_token(token, "refresh_secret".to_string())
}

/// Decode a token based on the a specified secret
fn decode_token(token: String, secret: String) -> Result<String, MyError> {
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_message) => Ok(token_message.claims.user_id),
        Err(e) => {
            return Err(MyError::General {
                desc: format!("{}", e),
            });
        }
    }
}
