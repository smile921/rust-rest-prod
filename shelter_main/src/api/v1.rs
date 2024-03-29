use std::ffi::OsString;
use std::sync::Arc;

use crate::state::ApplicationState;
use std::path::Path;

use super::handlers;
use super::middleware::jwt::auth;
use axum::routing::{get, post};
use axum::{middleware, Router};

pub fn configure(state: Arc<ApplicationState>) -> Router {
    let assets_dir = state.settings.load().assets_dir.clone();
    let dir: OsString = assets_dir.into();
    let _path = Path::new(dir.as_os_str());
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/dogs",
            post(handlers::dogs::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route("/dogs", get(handlers::dogs::list).with_state(state.clone()))
        .route(
            "/dogs/:id",
            get(handlers::dogs::get).with_state(state.clone()),
        )
        .route(
            "/login",
            post(handlers::login::login).with_state(state.clone()),
        )
    // .nest("/static", services::ServeDir::new(path))
}

use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::HttpBuilder;
use utoipa::openapi::security::SecurityScheme;
use utoipa::OpenApi;
use utoipa::{openapi, Modify};

#[derive(Debug, OpenApi)]
#[
    openapi(
        paths(
            handlers::hello::hello,
            handlers::login::login,
            handlers::dogs::create,
            handlers::dogs::get,
            handlers::dogs::list,
        ),
        components(
            schemas(
                crate::api::request::login::LoginRequest,
                crate::api::response::login::LoginResponse,
                crate::api::response::error::Status,
                crate::api::response::error::ErrorResponse,
                crate::api::response::dog::DogGetResponse,
                crate::api::response::dog::DogListResponse,
                crate::api::response::dog::DogCreateResponse,
                entity::dog::DogCreateRequest,
                entity::dog::Model,
            ),
        ),
        modifiers(&SecurityAddon),
        tags (
            (name = "hello", description = "Hello"),
            (name = "login", description = "Login"),
            (name = "dogs", description = "Dogs"),
        ),
        servers(
            (url  = "/v1", description = " Local server"),
        ),
    )
]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
