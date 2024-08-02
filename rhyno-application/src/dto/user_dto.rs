use rhyno_domain::{user::user_entity::User, value_objects::ValueObject};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserDTO {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl From<&User> for UserDTO {
    fn from(user: &User) -> Self {
        UserDTO {
            id: *user.id.value(),
            name: (&user.name).to_string(),
            email: (&user.email).to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct UserRegisterResponse {
    pub success: bool,
    pub message: String,
}
