use crate::modules::tickets::domain::{Ticket, TicketId};

pub trait TicketUseCase {
    fn get_by_id(&self, id: &TicketId) -> Result<Option<Ticket>, String>;
    fn create_ticket(&self, ticket: &Ticket) -> Result<(), String>;
}
