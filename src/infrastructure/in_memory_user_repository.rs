use crate::entities::user::User;
use crate::interfaces::user_repository::UserRepository;
use std::collections::HashMap;

pub struct InMemoryUserRepository {
    users: HashMap<u32, User>,
    next_id: u32,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_id: 1,
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn get_user_by_id(&self, id: u32) -> Option<User> {
        self.users.get(&id).cloned()
    }

    fn add_user(&mut self, mut user: User) {
        user.id = self.next_id;
        self.users.insert(self.next_id, user);
        self.next_id += 1;
    }
}
