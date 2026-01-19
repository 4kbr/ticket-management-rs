#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token(pub String);

impl Token {
    pub fn new<S: Into<String>>(s: S) -> Self { Token(s.into()) }
}
