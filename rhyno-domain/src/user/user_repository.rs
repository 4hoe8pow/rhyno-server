use crate::user::user_entity::User;

pub trait UserRepository {
    fn save(&self, user: &User) -> impl std::future::Future<Output = Result<(), String>> + Send;
}
