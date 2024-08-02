pub trait UserInputPort {
    async fn register_user(&self, id: u32, name: String, email: String) -> Result<(), String>;
}
