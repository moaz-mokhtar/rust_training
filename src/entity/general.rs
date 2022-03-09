use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Response<T> {
    pub message: String,
    pub data: T,
}
