use serde::Serialize;

pub mod controller;
pub mod presenter;

#[derive(Serialize)]
pub struct UserRegisterResponse {
    pub success: bool,
    pub message: String,
}
