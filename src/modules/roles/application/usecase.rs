use crate::modules::roles::domain::entity::Role;

pub trait RoleUseCase {
    fn get_by_name(&self, name: &str) -> Result<Option<Role>, String>;
    fn create_role(&self, role: &Role) -> Result<(), String>;
}
