use chrono::{DateTime, Utc};
use crate::modules::quotas::domain::value_object::QuotaId;

#[derive(Debug, Clone)]
pub struct Quota {
    pub id: QuotaId,
    pub name: String,
    pub limit: i64,
    pub used: i64,
    pub updated_at: DateTime<Utc>,
}

impl Quota {
    pub fn available(&self) -> i64 { self.limit - self.used }
}
