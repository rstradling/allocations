use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RosterItem {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub salary: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: Uuid,
    pub tag: String,
}

#[derive(Debug, Clone)]
pub struct Initiative {
    pub id: Uuid,
    pub initiative: String,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub id: Uuid,
    pub year: i64,
    pub jan: Option<f64>,
    pub feb: Option<f64>,
    pub mar: Option<f64>,
    pub apr: Option<f64>,
    pub may: Option<f64>,
    pub jun: Option<f64>,
    pub jul: Option<f64>,
    pub aug: Option<f64>,
    pub sep: Option<f64>,
    pub oct: Option<f64>,
    pub nov: Option<f64>,
    pub dec: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct RosterAssignment {
    pub id: Uuid,
    pub roster_id: Uuid,
    pub assignment_id: Uuid,
    pub allocation: f64,
    pub initiative_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct AssignmentTag {
    pub id: Uuid,
    pub assignment_id: Uuid,
    pub tag_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct RosterAssignmentTag {
    pub id: Uuid,
    pub roster_assignment_id: Uuid,
    pub tag_id: Uuid,
}
