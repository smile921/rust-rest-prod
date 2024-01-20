use std::sync::Arc;

use axum::{http::StatusCode, extract::State};

use crate::state::ApplicationState;

#[utoipa::path(
    get,
    path = "/hello",
    tag = "hello",
    responses (
        (status= 200, description = "Hello World", body = String),
    ),
)]
pub async fn hello(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\nHello world ! use configuration from {} \n\n",
        state.settings.load().config.location.clone().unwrap_or("-".to_string())
    ))
}