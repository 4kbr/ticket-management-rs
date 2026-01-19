use crate::modules::quotas::domain::value_object::QuotaId;

#[derive(Debug, Clone)]
pub struct QuotaDto {
    pub id: QuotaId,
    pub name: String,
    pub limit: i64,
    pub used: i64,
}
