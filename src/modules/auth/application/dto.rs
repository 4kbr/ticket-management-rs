#[derive(Debug, Clone)]
pub struct AuthenticateDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct TokenDto {
    pub token: String,
}
