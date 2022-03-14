use actix_web::{http::header::HeaderValue, HttpRequest};
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
                        _ => {
                            let error_message = format!("Wrong Authentication scheme");
                            Err(MyError::from(error_message))
                        }
                    }
                }
                None => {
                    let error_message = format!("Empty `Authorization` schema");
                    Err(MyError::from(error_message))
                }
            }
        }
        // No `Authorization` header found
        None => {
            let error_message = format!("No `Authorization` header found");
            Err(MyError::from(error_message))
        }
    }
}

pub fn decode_access_token(token: String) -> Result<String, MyError> {
    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("access_secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .unwrap();

    Ok(token_message.claims.user_id)
}

pub fn decode_refresh_token(token: String) -> Result<String, MyError> {
    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("refresh_secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .unwrap();

    Ok(token_message.claims.user_id)
}

pub fn parse_refresh_token_from_cookies(cookie_header: &str) -> Option<String> {
    let cookies: Vec<(&str, &str)> = cookie_header
        .split("; ")
        .map(|raw| {
            let parsed = cookie::Cookie::parse(raw)
                .map(|c| (c.name_raw().unwrap(), c.value_raw().unwrap()))
                .unwrap();
            parsed
        })
        .collect();

    let refresh_token = cookies.iter().find_map(|(c_header, c_value)| {
        if *c_header == "refresh_token" {
            Some(c_value.to_string())
        } else {
            None
        }
    });

    refresh_token
}
