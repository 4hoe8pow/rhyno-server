use crate::applications::dto::user_dto::{UserDTO, UserRegisterResponse};
use crate::RegistryContainer;
use axum::{http::StatusCode, Extension, Json};
use std::sync::Arc;

pub async fn register_user(
    state: Extension<Arc<RegistryContainer>>,
    Json(user_dto): Json<UserDTO>,
) -> (StatusCode, Json<UserRegisterResponse>) {
    let controller = &state.0.controller;

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
