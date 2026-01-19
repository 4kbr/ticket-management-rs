use crate::modules::users::domain::value_object::SecureId;
use crate::modules::tickets::domain::value_object::TicketId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone)]
pub struct Ticket {
    pub id: TicketId,
    pub title: String,
    pub description: Option<String>,
    pub reporter: SecureId,
    pub assignee: Option<SecureId>,
    pub status: TicketStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Ticket {
    pub fn new(id: TicketId, title: String, reporter: SecureId) -> Self {
        let now = Utc::now();
        Self { id, title, description: None, reporter, assignee: None, status: TicketStatus::Open, created_at: now, updated_at: now }
    }
}
