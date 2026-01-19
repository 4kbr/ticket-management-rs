use crate::modules::auth::entity::AccessToken;
use crate::modules::users::value_object::SecureId;

pub trait AuthRepository {
    fn store_token(&self, token: &AccessToken) -> Result<(), String>;
    fn revoke_token(&self, token: &str) -> Result<(), String>;
    fn find_token_by_user(&self, user_id: &SecureId) -> Result<Option<AccessToken>, String>;
}
