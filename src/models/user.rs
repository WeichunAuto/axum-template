use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub active: bool,
}
