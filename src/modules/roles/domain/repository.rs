use crate::modules::roles::domain::entity::Role;

pub trait RoleRepository {
    fn find_by_name(&self, name: &str) -> Result<Option<Role>, String>;
    fn save(&self, role: &Role) -> Result<(), String>;
}
