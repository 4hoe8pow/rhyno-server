use rhyno_application::{
    dto::user_dto::{UserDTO, UserRegisterResponse},
    user_usecase::user_input_port::UserInputPort,
};

pub struct UserController<I: UserInputPort> {
    input_port: I,
}

impl<I: UserInputPort> UserController<I> {
    pub fn new(input_port: I) -> Self {
        Self { input_port }
    }

    pub async fn register(&self, user_dto: UserDTO) -> Result<UserRegisterResponse, String> {
        self.input_port
            .register_user(user_dto.id, user_dto.name, user_dto.email)
            .await
            .map(|_| UserRegisterResponse {
                success: true,
                message: "User registered successfully.".to_string(),
            })
            .map_err(|e| e)
    }
}
