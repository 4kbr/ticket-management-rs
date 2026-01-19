#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecureId(pub String);

impl SecureId {
    pub fn new<S: Into<String>>(s: S) -> Self { SecureId(s.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}
