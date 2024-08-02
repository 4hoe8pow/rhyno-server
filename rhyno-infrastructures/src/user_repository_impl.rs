use rhyno_domain::user::{user_entity::User, user_repository::UserRepository};

pub struct UserRepositoryImpl;

impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: &User) -> Result<(), String> {
        println!("User saved: {} - {}", user.id, user.name);
        Ok(())
    }
}
