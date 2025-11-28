use crate::dao;
use domain::dto;

impl From<&dto::Employee> for dao::Employee {
    fn from(ri: &dto::Employee) -> Self {
        dao::Employee {
            id: ri.id,
            first_name: ri.first_name.clone(),
            last_name: ri.last_name.clone(),
            email: ri.email.clone(),
            salary: ri.salary.clone(),
        }
    }
}

impl From<&dao::Employee> for dto::Employee {
    fn from(ri: &dao::Employee) -> Self {
        dto::Employee {
            id: ri.id,
            first_name: ri.first_name.clone(),
            last_name: ri.last_name.clone(),
            email: ri.email.clone(),
            salary: ri.salary.clone(),
        }
    }
}
