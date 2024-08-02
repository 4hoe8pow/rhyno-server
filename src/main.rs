mod container;
mod handlers;

use crate::container::RegistryContainer;
use crate::handlers::*;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let state = Arc::new(RegistryContainer::new());

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/register", post(register_user))
        .layer(Extension(state));

    Ok(router.into())
}
