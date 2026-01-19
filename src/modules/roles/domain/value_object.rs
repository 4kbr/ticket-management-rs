#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permission(pub String);

impl Permission {
    pub fn new<S: Into<String>>(s: S) -> Self { Permission(s.into()) }
}
