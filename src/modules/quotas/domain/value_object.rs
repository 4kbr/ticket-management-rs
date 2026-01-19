#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuotaId(pub String);

impl QuotaId { pub fn new<S: Into<String>>(s: S) -> Self { QuotaId(s.into()) } }
