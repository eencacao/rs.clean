use crate::entities::user::User;
use crate::interfaces::user_repository::UserRepository;
use std::collections::HashMap;

pub struct InMemoryUserRepository {
    users: HashMap<u32, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn get_user_by_id(&self, id: u32) -> Option<User> {
        self.users.get(&id).cloned()
    }

    fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
}
