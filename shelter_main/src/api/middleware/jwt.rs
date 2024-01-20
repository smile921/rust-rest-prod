use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::header;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Json;

use crate::api::response::error::ErrorResponse;
use crate::api::response::error::Status;
use jsonwebtoken::Validation; 
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;


pub async fn auth(State(state): State<Arc<ApplicationState>>,
    mut req: Request<Body>,
    next: Next,
    ) ->  Result<impl IntoResponse,(StatusCode,Json<ErrorResponse>)>
    {
        let token = req.headers()
            .get(header::AUTHORIZATION)
            .and_then(|auth_header| auth_header.to_str().ok())
            .and_then(|auth_value| {
                auth_value
                    .strip_prefix("Bearer ")
                    .map(|stripped| stripped.to_owned())
            });
        let token = token.ok_or_else(|| {
            let json_error = ErrorResponse {
                status: Status::Error,
                message: "Missing bearer token".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(json_error))
        })?;

        let secret = &state.settings.load().token_secret;
        let claims = decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| {
            let json_error = ErrorResponse {
                status: Status::Error,
                message: "Invalid bearer token".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(json_error))
        })?
        .claims;
    
        req.extensions_mut().insert(claims);
        
        let req_next = req;
        Ok(next.run(req_next).await)
}

// 894- 776