use rhyno_application::{dto::user_dto::UserDTO, user_usecase::user_output_port::UserOutputPort};

pub struct UserPresenter;

impl UserOutputPort for UserPresenter {
    fn show_user(&self, user_dto: &UserDTO) {
        println!(
            "User registered: {} : {} ({})",
            user_dto.id, user_dto.name, user_dto.email
        );
    }
}
