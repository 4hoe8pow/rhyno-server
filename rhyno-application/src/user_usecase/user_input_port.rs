pub trait UserInputPort {
    fn register_user(
        &self,
        id: u32,
        name: String,
        email: String,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
}
