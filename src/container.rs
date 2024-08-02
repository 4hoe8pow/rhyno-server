use crate::applications::user_usecase::user_interactor::UserInteractor;
use crate::domains::user::user_factory::DefaultUserFactory;
use crate::infrastructures::user_repository_impl::UserRepositoryImpl;
use crate::presentations::controller::UserController;
use crate::presentations::presenter::UserPresenter;

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
