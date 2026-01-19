use crate::modules::roles::domain::entity::Role;

pub trait RoleDomainService {
    fn can_assign_role(&self, role: &Role) -> Result<(), String>;
}
