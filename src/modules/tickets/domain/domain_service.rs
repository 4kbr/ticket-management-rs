use crate::modules::tickets::domain::{Ticket, TicketStatus};

pub trait TicketDomainService {
    fn can_change_status(&self, ticket: &Ticket, new_status: &TicketStatus) -> Result<(), String>;
}
