use crate::modules::users::domain::value_object::SecureId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub secure_id: SecureId,
    pub email: String,
    pub full_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub permissions: Vec<String>,
}

impl User {
    pub fn new(secure_id: SecureId, email: String) -> Self {
        let now = Utc::now();
        Self {
            secure_id,
            email,
            full_name: None,
            is_active: true,
            created_at: now,
            updated_at: now,
            permissions: vec![],
        }
    }
}
