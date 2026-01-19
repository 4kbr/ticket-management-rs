use crate::modules::quotas::domain::{Quota, QuotaId};

pub trait QuotaUseCase {
    fn get_by_id(&self, id: &QuotaId) -> Result<Option<Quota>, String>;
    fn consume(&self, id: &QuotaId, amount: i64) -> Result<(), String>;
}
