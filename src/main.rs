mod container;
mod handlers;

use crate::container::RegistryContainer;
use crate::handlers::*;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
struct DBState {
    _pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    let container = Arc::new(RegistryContainer::new());
    let state = DBState { _pool: pool };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/register", post(register_user))
        .layer(Extension(container))
        .with_state(state);

    Ok(router.into())
}
