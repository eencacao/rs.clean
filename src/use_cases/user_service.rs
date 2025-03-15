use crate::entities::user::User;
use crate::interfaces::user_repository::UserRepository;

pub struct UserService<T: UserRepository> {
    repo: T,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub fn get_user(&self, id: u32) -> Option<User> {
        self.repo.get_user_by_id(id)
    }

    pub fn create_user(&mut self, id: u32, name: String, email: String) {
        let user = User::new(id, name, email);
        self.repo.add_user(user);
    }
}
