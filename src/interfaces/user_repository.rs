use crate::entities::user::User;

pub trait UserRepository {
    fn get_user_by_id(&self, id: u32) -> Option<User>;
    fn add_user(&mut self, user: User);
}
