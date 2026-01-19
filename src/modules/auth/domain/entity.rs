use crate::modules::users::value_object::SecureId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AccessToken {
    pub token: String,
    pub user_id: SecureId,
    pub expires_at: DateTime<Utc>,
}

impl AccessToken {
    pub fn is_expired(&self) -> bool { Utc::now() > self.expires_at }
}
