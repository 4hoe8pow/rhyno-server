use crate::RegistryContainer;
use axum::{http::StatusCode, Extension, Json};
use rhyno_application::dto::user_dto::UserDTO;
use rhyno_presentation::UserRegisterResponse;
use std::sync::Arc;

pub async fn register_user(
    container: Extension<Arc<RegistryContainer>>,
    Json(user_dto): Json<UserDTO>,
) -> (StatusCode, Json<UserRegisterResponse>) {
    let controller = &container.0.controller;

    match controller.register(user_dto).await {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UserRegisterResponse {
                success: false,
                message: e,
            }),
        ),
    }
}

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}
