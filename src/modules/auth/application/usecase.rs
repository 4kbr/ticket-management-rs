use crate::modules::users::domain::value_object::SecureId;
use crate::modules::auth::domain::entity::AccessToken;

pub trait AuthServiceUseCase {
    fn authenticate(&self, email: &str, password: &str) -> Result<AccessToken, String>;
    fn refresh(&self, user_id: &SecureId, refresh_token: &str) -> Result<AccessToken, String>;
}
