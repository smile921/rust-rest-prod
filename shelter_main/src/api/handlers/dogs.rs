
use crate::api::middleware::json::CustomJson;
use crate::api::response::TokenClaims;
use crate::api::response::dog::DogCreateResponse;
use crate::api::response::error::AppError;
use crate::api::response::error::Status;
use crate::state::ApplicationState;

use axum::Extension;
use axum::Json; 
use axum::extract::State;
use entity::dog::DogCreateRequest;
use sea_orm::ActiveModelTrait;
use sea_orm::IntoActiveModel;
use sea_orm::TryIntoModel;
use std::sync::Arc;


pub async fn create(
        Extension(_claims): Extension<TokenClaims>,
        State(state):State<Arc<ApplicationState>>,
        CustomJson(payload): CustomJson<DogCreateRequest>
    ) ->  Result<Json<DogCreateResponse>,AppError>{
        let dog_active_model = payload.into_active_model();
        let dog_model = dog_active_model.save(state.db_conn.load().as_ref()).await?;
        let dog = dog_model.try_into_model()?;

        let response = DogCreateResponse {
            status: Status::Success,
            data: Some(dog),
        };
        Ok(Json(response))
    }