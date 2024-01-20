use entity::dog::Model;
use serde::Serialize;
use core::option::Option;

use super::error::Status;

#[derive(Serialize,Debug)]
pub struct DogCreateResponse {
    pub status: Status,
    pub data: Option<Model>,
}