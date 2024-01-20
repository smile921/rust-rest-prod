use serde::Serialize;
use utoipa::ToSchema;

use super::error::Status;

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub status: Status,
    pub token: String,
}
