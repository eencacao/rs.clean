use crate::entities::todo::Todo;

pub trait TodoRepository {
    fn get_all(&self) -> Vec<Todo>;
    fn get_by_id(&self, id: u32) -> Option<Todo>;
    fn save(&mut self, title: String) -> Todo;
    fn update(
        &mut self,
        id: u32,
        title: String,
        done: bool,
    ) -> Option<Todo>;
    fn delete(&mut self, id: u32) -> bool;
}
