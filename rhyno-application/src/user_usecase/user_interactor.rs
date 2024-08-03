use crate::dto::user_dto::UserDTO;
use rhyno_domain::user::{user_factory::UserFactory, user_repository::UserRepository};

use super::{user_input_port::UserInputPort, user_output_port::UserOutputPort};

pub struct UserInteractor<R: UserRepository, O: UserOutputPort, F: UserFactory> {
    repository: R,
    output: O,
    factory: F,
}

impl<R: UserRepository, O: UserOutputPort, F: UserFactory> UserInteractor<R, O, F> {
    pub fn new(repository: R, output: O, factory: F) -> Self {
        UserInteractor {
            repository,
            output,
            factory,
        }
    }
}

impl<
        R: UserRepository + Send + Sync,
        O: UserOutputPort + Send + Sync,
        F: UserFactory + Send + Sync,
    > UserInputPort for UserInteractor<R, O, F>
{
    fn register_user(
        &self,
        id: u32,
        name: String,
        email: String,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send {
        let repository = &self.repository;
        let output = &self.output;
        let factory = &self.factory;

        async move {
            let user = factory
                .create(id, name, email)
                .map_err(|e| format!("Failed to create user: {}", e))?;

            repository
                .save(&user)
                .await
                .map_err(|e| format!("Failed to save user: {}", e))?;

            let user_dto = UserDTO::from(&user);
            output.show_user(&user_dto);

            Ok(())
        }
    }
}
