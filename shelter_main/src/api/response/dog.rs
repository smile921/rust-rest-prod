use entity::dog::Model;
use serde::Serialize;
use core::option::Option;

#[derive(Serialize,Debug)]
pub struct DogCreateResponse {
    pub status: String,
    pub data: Option<Model>,
}