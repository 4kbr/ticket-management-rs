#[derive(Debug, Clone)]
pub struct RoleDto {
    pub name: String,
    pub permissions: Vec<String>,
}
