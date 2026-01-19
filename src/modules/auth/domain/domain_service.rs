use crate::modules::auth::entity::AccessToken;
use crate::modules::users::value_object::SecureId;

pub trait AuthService {
    fn authenticate(&self, email: &str, password: &str) -> Result<AccessToken, String>;
    fn refresh(&self, user_id: &SecureId, refresh_token: &str) -> Result<AccessToken, String>;
}
