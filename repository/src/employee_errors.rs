use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CreateEmployeeError {
    #[error("Failed to create employee with id {id}: {source}")]
    CreateFailed { id: Uuid, source: sqlx::Error },

    #[error("Duplicate pet with the same id: {id}: {source}")]
    Duplicate { id: String, source: sqlx::Error },

    #[error("Failed to commit transactions: {source}")]
    CommitFailed { source: sqlx::Error },

    #[error("Unknown error for employee: {source}")]
    Unknown { source: sqlx::Error },
}

impl From<CreateEmployeeError> for sqlx::Error {
    fn from(error: CreateEmployeeError) -> Self {
        match error {
            CreateEmployeeError::CreateFailed { id: _, source } => source,
            CreateEmployeeError::Duplicate { id: _, source } => source,
            CreateEmployeeError::CommitFailed { source } => source,
            CreateEmployeeError::Unknown { source } => source,
        }
    }
}

impl CreateEmployeeError {
    pub fn from_sqlx_commit_failed(error: sqlx::Error) -> Self {
        CreateEmployeeError::CommitFailed { source: error }
    }
    pub fn from_sqlx_with_email(error: sqlx::Error, email: &str) -> Self {
        if let Some(db_error) = error.as_database_error() {
            if let Some(code) = db_error.code() {
                if code == "23505" {
                    return CreateEmployeeError::Duplicate {
                        id: email.to_string(),
                        source: error,
                    };
                }
            }
        }
        CreateEmployeeError::Unknown { source: error }
    }
}

#[derive(Debug, Error)]
pub enum UpdateEmployeeError {
    #[error("Failed to commit transactions: {source}")]
    CommitFailed { source: sqlx::Error },

    #[error("Unknown error for employee: {source}")]
    Unknown { source: sqlx::Error },
}

impl From<UpdateEmployeeError> for sqlx::Error {
    fn from(error: UpdateEmployeeError) -> Self {
        match error {
            UpdateEmployeeError::CommitFailed { source } => source,
            UpdateEmployeeError::Unknown { source } => source,
        }
    }
}

#[derive(Debug, Error)]
pub enum DeleteEmployeeError {
    #[error("Unknown error for employee: {source}")]
    Unknown { source: sqlx::Error },
}
impl From<DeleteEmployeeError> for sqlx::Error {
    fn from(error: DeleteEmployeeError) -> Self {
        match error {
            DeleteEmployeeError::Unknown { source } => source,
        }
    }
}

#[derive(Debug, Error)]
pub enum GetEmployeesError {
    #[error("Unknown error for employees: {source}")]
    Unknown { source: sqlx::Error },
}

impl From<GetEmployeesError> for sqlx::Error {
    fn from(error: GetEmployeesError) -> Self {
        match error {
            GetEmployeesError::Unknown { source } => source,
        }
    }
}

#[derive(Debug, Error)]
pub enum GetEmployeeError {
    #[error("Unknown error for employee: {source}")]
    Unknown { source: sqlx::Error },
}

impl From<GetEmployeeError> for sqlx::Error {
    fn from(error: GetEmployeeError) -> Self {
        match error {
            GetEmployeeError::Unknown { source } => source,
        }
    }
}
