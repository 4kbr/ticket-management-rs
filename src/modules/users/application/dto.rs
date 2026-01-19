use crate::modules::users::domain::value_object::SecureId;

#[derive(Debug, Clone)]
pub struct CreateUserDto {
    pub email: String,
    pub full_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateUserDto {
    pub full_name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct UserDto {
    pub secure_id: SecureId,
    pub email: String,
    pub full_name: Option<String>,
}
