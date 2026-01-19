#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TicketId(pub String);

impl TicketId {
    pub fn new<S: Into<String>>(s: S) -> Self { TicketId(s.into()) }
}
