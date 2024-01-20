use axum::{response::{IntoResponse,Response}, http::StatusCode, Json};
use serde::Serialize;
use utoipa::ToSchema;


#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                ErrorResponse {
                    status: Status::Error,
                    message: self.0.to_string(),
                }
            )
        ).into_response()
    }
}

impl <E> From<E> for AppError
where 
    E: Into<anyhow::Error>, {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}