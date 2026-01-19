use crate::modules::tickets::domain::value_object::TicketId;

#[derive(Debug, Clone)]
pub struct CreateTicketDto {
    pub title: String,
    pub description: Option<String>,
    pub reporter_id: TicketId,
}
