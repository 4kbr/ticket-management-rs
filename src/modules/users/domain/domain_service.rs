use crate::modules::users::domain::entity::User;

pub trait UserDomainService {
    fn can_create_user(&self, user: &User) -> Result<(), String>;
}
