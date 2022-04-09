// ===

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisterationRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgotRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetRequest {
    pub token: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn as_dto(&self) -> UserDTO {
        UserDTO {
            id: self.id.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable)]
#[table_name = "user_token"]
pub struct UserToken {
    pub user_id: Uuid,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable)]
#[table_name = "reset"]
pub struct Reset {
    pub token: String,
    pub email: String,
}
