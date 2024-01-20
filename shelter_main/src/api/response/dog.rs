use entity::dog::Model;
use serde::Serialize;
use utoipa::ToSchema;
use core::option::Option;

use super::error::Status;

#[derive(Serialize,Debug,ToSchema)]
pub struct DogCreateResponse {
    pub status: Status,
    pub data: Option<Model>,
}

#[derive(Serialize,Debug,ToSchema)]
pub struct DogListResponse {
    pub status: Status,
    pub data: Vec<Model>,
}

#[derive(Serialize,Debug,ToSchema)]
pub struct DogGetResponse {
    pub status: Status,
    pub data: Option<Model>,
}