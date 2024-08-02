use crate::user::user_entity::User;

pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<(), String>;
}
