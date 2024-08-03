use rhyno_domain::user::{user_entity::User, user_repository::UserRepository};

pub struct UserRepositoryImpl;

impl UserRepository for UserRepositoryImpl {
    fn save(&self, user: &User) -> impl std::future::Future<Output = Result<(), String>> + Send {
        async move {
            println!("User saved: {} - {}", user.id, user.name);
            Ok(())
        }
    }
}
