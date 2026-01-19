use crate::modules::users::domain::{User, SecureId};

pub trait CreateUserUseCase {
    fn execute(&self, dto: CreateUserDto) -> Result<User, String>;
}

pub trait GetUserUseCase {
    fn get_by_id(&self, id: &SecureId) -> Result<Option<User>, String>;
}
