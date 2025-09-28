use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct UserResponse {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

#[derive(Deserialize)]
pub(crate) struct User {
    pub name: String,
    pub password: String,
}
