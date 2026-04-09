use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use identity_application::{ports::RegisterUserPort, use_cases::RegisterUserCommand};
use serde::{Deserialize, Serialize};

use crate::errors::ApiError;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMakerRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMakerResponse {
    pub user_id: String,
    pub email: String,
    pub role: String,
}

pub async fn register_maker(
    State(uc): State<Arc<dyn RegisterUserPort>>,
    Json(body): Json<RegisterMakerRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let command = RegisterUserCommand::new(body.email.clone(), body.password, "Maker".into())
        .map_err(ApiError::from)?;

    let (id, _event) = uc.execute(command).await.map_err(ApiError::from)?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterMakerResponse {
            user_id: id.0.to_string(),
            email: body.email,
            role: "Maker".into(),
        }),
    ))
}
