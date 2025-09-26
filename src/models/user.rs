use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}
