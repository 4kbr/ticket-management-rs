use crate::modules::tickets::domain::entity::Ticket;
use crate::modules::tickets::domain::value_object::TicketId;

pub trait TicketRepository {
    fn find_by_id(&self, id: &TicketId) -> Result<Option<Ticket>, String>;
    fn save(&self, ticket: &Ticket) -> Result<(), String>;
}
