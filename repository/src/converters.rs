use crate::dao;
use domain::dto;

impl From<&dto::RosterItem> for dao::RosterItem {
    fn from(ri: &dto::RosterItem) -> Self {
        dao::RosterItem {
            id: ri.id,
            first_name: ri.first_name.clone(),
            last_name: ri.last_name.clone(),
            email: ri.email.clone(),
            salary: ri.salary.clone(),
        }
    }
}

impl From<&dao::RosterItem> for dto::RosterItem {
    fn from(ri: &dao::RosterItem) -> Self {
        dto::RosterItem {
            id: ri.id,
            first_name: ri.first_name.clone(),
            last_name: ri.last_name.clone(),
            email: ri.email.clone(),
            salary: ri.salary.clone(),
        }
    }
}
