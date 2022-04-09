use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ObjectResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageResponse<T> {
    pub message: T,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenResponse<T> {
    pub token: T,
}
