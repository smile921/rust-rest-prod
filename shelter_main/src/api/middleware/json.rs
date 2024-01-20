use axum::async_trait;
use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::extract::Request;
use axum::http::StatusCode;
use serde_json::json;
use serde_json::Value;

use crate::api::response::error::Status;

pub struct CustomJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for CustomJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err((
                rejection.status(),
                axum::Json(json!({
                    "status": Status::Error,
                    "message": rejection.body_text(),
                })),
            )),
        }
    }
}
