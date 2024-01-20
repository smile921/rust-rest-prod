use axum::Router;

mod v1;
mod handlers;

pub fn configure() -> Router {
    Router::new().nest("/v1", v1::configure())
}