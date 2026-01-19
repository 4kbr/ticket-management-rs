use crate::modules::quotas::domain::entity::Quota;
use crate::modules::quotas::domain::value_object::QuotaId;

pub trait QuotaRepository {
    fn find_by_id(&self, id: &QuotaId) -> Result<Option<Quota>, String>;
    fn save(&self, quota: &Quota) -> Result<(), String>;
}
