use crate::http::api::{ApiError, ApiSuccess};
use crate::models::employee::*;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use repository::employee_repo::EmployeeRepo;
use uuid::Uuid;

use crate::http::AppState;

pub async fn create_employee<RR: EmployeeRepo>(
    State(state): State<AppState<RR>>,
    Json(body): Json<CreateEmployeeRequest>,
) -> Result<ApiSuccess<CreateEmployeeResponse>, ApiError> {
    let domain_req = (&body).into();
    state
        .employee_repo
        .create(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref ri| ApiSuccess::new(StatusCode::CREATED, ri.into()))
}

pub async fn update_employee<RR: EmployeeRepo>(
    State(state): State<AppState<RR>>,
    Json(body): Json<UpdateEmployeeRequest>,
) -> Result<ApiSuccess<UpdateEmployeeResponse>, ApiError> {
    let domain_req = (&body).into();
    state
        .employee_repo
        .update(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref pet| ApiSuccess::new(StatusCode::CREATED, pet.into()))
}

pub async fn get_employee<RR: EmployeeRepo>(
    State(state): State<AppState<RR>>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<GetEmployeeResponse>, ApiError> {
    let employee_id = uuid::Uuid::parse_str(&id).map_err(ApiError::from)?;
    match state.employee_repo.get(employee_id).await {
        Ok(Some(ri)) => {
            let response_data = GetEmployeeResponse::from(&ri);
            Ok(ApiSuccess::new(StatusCode::OK, response_data))
        }
        Ok(None) => {
            // Handle not found case
            Err(ApiError::NotFound("Employee item not found".to_string()))
        }
        Err(e) => Err(ApiError::from(e)),
    }
}
pub async fn delete_employee<RR: EmployeeRepo>(
    State(state): State<AppState<RR>>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<DeleteEmployeeResponse>, ApiError> {
    // Handle the Result<Option<Pet>, GetPetError>
    let rid = Uuid::parse_str(&id).map_err(ApiError::from)?;
    match state.employee_repo.delete(rid).await {
        Ok(()) => {
            let response_data = DeleteEmployeeResponse {};
            Ok(ApiSuccess::new(StatusCode::OK, response_data))
        }
        Err(e) => Err(ApiError::from(e)),
    }
}

pub async fn get_employees<RR: EmployeeRepo>(
    State(state): State<AppState<RR>>,
) -> Result<ApiSuccess<Vec<GetEmployeeResponse>>, ApiError> {
    // Handle the Result<Option<Pet>, GetPetError>
    match state.employee_repo.get_all().await {
        Ok(items) => {
            let response_data = items
                .into_iter()
                .map(|x| GetEmployeeResponse::from(&x))
                .collect();
            Ok(ApiSuccess::new(StatusCode::OK, response_data))
        }
        Err(e) => Err(ApiError::from(e)),
    }
}
