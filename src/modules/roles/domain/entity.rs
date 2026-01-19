#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub permissions: Vec<String>,
}

impl Role {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            permissions: vec![],
        }
    }
}
