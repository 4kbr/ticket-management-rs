use crate::modules::quotas::domain::entity::Quota;

pub trait QuotaDomainService {
    fn check_and_consume(&self, quota: &mut Quota, amount: i64) -> Result<(), String>;
}
