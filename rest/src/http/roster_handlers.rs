use crate::http::api::{ApiError, ApiSuccess};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use bigdecimal::BigDecimal;
use domain::dto;
use repository::roster_repo::{RosterItemError, RosterRepo};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::http::AppState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetRosterItemResponse {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    #[serde(with = "bigdecimal::serde::json_num")]
    salary: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateRosterItemRequest {
    first_name: String,
    last_name: String,
    email: String,
    salary: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRosterItemRequest {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    salary: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateRosterItemResponse {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    salary: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateRosterItemResponse {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    salary: BigDecimal,
}

impl From<&CreateRosterItemRequest> for dto::RosterItem {
    fn from(value: &CreateRosterItemRequest) -> Self {
        dto::RosterItem {
            id: uuid::Uuid::nil(), // This will get replaced properly on down the line
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}

impl From<&UpdateRosterItemRequest> for dto::RosterItem {
    fn from(value: &UpdateRosterItemRequest) -> Self {
        dto::RosterItem {
            id: value.id,
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}

impl From<&dto::RosterItem> for UpdateRosterItemResponse {
    fn from(value: &dto::RosterItem) -> Self {
        UpdateRosterItemResponse {
            id: value.id, // This will get replaced properly on down the line
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}

impl From<&dto::RosterItem> for GetRosterItemResponse {
    fn from(value: &dto::RosterItem) -> Self {
        GetRosterItemResponse {
            id: value.id, // This will get replaced properly on down the line
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}

impl From<&dto::RosterItem> for CreateRosterItemResponse {
    fn from(value: &dto::RosterItem) -> Self {
        CreateRosterItemResponse {
            id: value.id, // This will get replaced properly on down the line
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}

impl From<uuid::Error> for ApiError {
    fn from(error: uuid::Error) -> Self {
        ApiError::NotFound(format!("Invalid UUID: {}", error))
    }
}

impl From<RosterItemError> for ApiError {
    fn from(e: RosterItemError) -> Self {
        match e {
            RosterItemError::CreateFailed { id, source } => Self::UnprocessableEntity(format!(
                "roster item with id {} already exists from source {}",
                id, source
            )),
            RosterItemError::Duplicate { id, source } => Self::InternalServerError(format!(
                "Duplicate error updating or creating a roster_item {} from source {}",
                id, source
            )),
            RosterItemError::Unknown { id, source } => Self::InternalServerError(format!(
                "Unable to process roster_item with id {} from source {}",
                id, source
            )),
            RosterItemError::CommitFailed { source } => Self::InternalServerError(format!(
                "CommitFailed for creating a roster_item at {}",
                source
            )),
            RosterItemError::Sqlx(source) => Self::InternalServerError(format!(
                "Sqlx error for creating a roster_item at {}",
                source
            )),
        }
    }
}

pub async fn create_roster_item<RR: RosterRepo>(
    State(state): State<AppState<RR>>,
    Json(body): Json<CreateRosterItemRequest>,
) -> Result<ApiSuccess<CreateRosterItemResponse>, ApiError> {
    let domain_req = (&body).into();
    state
        .roster_repo
        .create(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref ri| ApiSuccess::new(StatusCode::CREATED, ri.into()))
}

pub async fn update_roster_item<RR: RosterRepo>(
    State(state): State<AppState<RR>>,
    Json(body): Json<UpdateRosterItemRequest>,
) -> Result<ApiSuccess<UpdateRosterItemResponse>, ApiError> {
    let domain_req = (&body).into();
    state
        .roster_repo
        .update(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref pet| ApiSuccess::new(StatusCode::CREATED, pet.into()))
}

pub async fn get_roster_item<RR: RosterRepo>(
    State(state): State<AppState<RR>>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<GetRosterItemResponse>, ApiError> {
    let roster_item_id = uuid::Uuid::parse_str(&id).map_err(ApiError::from)?;
    match state.roster_repo.get(roster_item_id).await {
        Ok(Some(ri)) => {
            let response_data = GetRosterItemResponse::from(&ri);
            Ok(ApiSuccess::new(StatusCode::OK, response_data))
        }
        Ok(None) => {
            // Handle not found case
            Err(ApiError::NotFound("Roster item not found".to_string()))
        }
        Err(e) => Err(ApiError::from(e)),
    }
}
/*pub async fn delete_roster_item<RR: RosterRepo>(
    State(state): State<AppState<RR>>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<GetPetResponseData>, ApiError> {
    // Handle the Result<Option<Pet>, GetPetError>
    match state.pet_service.delete(pet_id).await {
        Ok(Some(pet)) => {
            let response_data = GetPetResponseData::from(&pet);
            Ok(ApiSuccess::new(StatusCode::OK, response_data))
        }
        Ok(None) => {
            // Handle not found case
            let response_data =
                GetPetResponseData::new(uuid::Uuid::nil().to_string(), "".to_string());
            Ok(ApiSuccess::new(StatusCode::NOT_FOUND, response_data))
        }
        Err(e) => Err(ApiError::from(e)),
    }
}

pub async fn get_all_roster_items<RR: RosterRepo>(
    State(state): State<AppState<RR>>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<GetPetResponseData>, ApiError> {
    let pet_id = PetId::new(id);

    // Handle the Result<Option<Pet>, GetPetError>
    match state.roster_repo.get_all().await {
        Ok(Some(pet)) => {
            let response_data = GetPetResponseData::from(&pet);
            Ok(ApiSuccess::new(StatusCode::OK, response_data))
        }
        Ok(None) => {
            // Handle not found case
            let response_data =
                GetPetResponseData::new(uuid::Uuid::nil().to_string(), "".to_string());
            Ok(ApiSuccess::new(StatusCode::NOT_FOUND, response_data))
        }
        Err(e) => Err(ApiError::from(e)),
    }
}*/
