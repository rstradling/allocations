use bigdecimal::BigDecimal;
use domain::dto;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetEmployeeResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    email: String,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub salary: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeleteEmployeeResponse {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub salary: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateEmployeeRequest {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub salary: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateEmployeeResponse {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    salary: BigDecimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateEmployeeResponse {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    salary: BigDecimal,
}

impl From<&CreateEmployeeRequest> for dto::Employee {
    fn from(value: &CreateEmployeeRequest) -> Self {
        dto::Employee {
            id: uuid::Uuid::nil(), // This will get replaced properly on down the line
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}

impl From<&UpdateEmployeeRequest> for dto::Employee {
    fn from(value: &UpdateEmployeeRequest) -> Self {
        dto::Employee {
            id: value.id,
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}

impl From<&dto::Employee> for UpdateEmployeeResponse {
    fn from(value: &dto::Employee) -> Self {
        UpdateEmployeeResponse {
            id: value.id, // This will get replaced properly on down the line
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}

impl From<&dto::Employee> for GetEmployeeResponse {
    fn from(value: &dto::Employee) -> Self {
        GetEmployeeResponse {
            id: value.id, // This will get replaced properly on down the line
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}

impl From<&dto::Employee> for CreateEmployeeResponse {
    fn from(value: &dto::Employee) -> Self {
        CreateEmployeeResponse {
            id: value.id, // This will get replaced properly on down the line
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
            salary: value.salary.clone(),
        }
    }
}
