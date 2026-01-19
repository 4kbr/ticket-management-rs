use crate::modules::users::domain::entity::User;
use crate::modules::users::domain::value_object::SecureId;

pub trait UserRepository {
    fn find_by_secure_id(&self, id: &SecureId) -> Result<Option<User>, String>;
    fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    fn save(&self, user: &User) -> Result<(), String>;
}
