use rhyno_application::user_usecase::user_interactor::UserInteractor;
use rhyno_domain::user::user_factory::DefaultUserFactory;
use rhyno_infrastructures::user_repository_impl::UserRepositoryImpl;
use rhyno_presentation::{controller::UserController, presenter::UserPresenter};

pub struct RegistryContainer {
    pub controller:
        UserController<UserInteractor<UserRepositoryImpl, UserPresenter, DefaultUserFactory>>,
}

impl RegistryContainer {
    pub fn new() -> Self {
        let user_repository = UserRepositoryImpl;
        let user_presenter = UserPresenter;
        let user_factory = DefaultUserFactory;
        let user_interactor = UserInteractor::new(user_repository, user_presenter, user_factory);
        let controller = UserController::new(user_interactor);

        Self { controller }
    }
}
