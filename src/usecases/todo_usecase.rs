use crate::entities::todo::Todo;
use crate::interfaces::todo_repository::TodoRepository;

pub struct TodoUseCase<R: TodoRepository> {
    pub repo: R,
}

impl<R: TodoRepository> TodoUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn get_all(&self) -> Vec<Todo> {
        self.repo.get_all()
    }

    pub fn get_by_id(&self, id: u32) -> Option<Todo> {
        self.repo.get_by_id(id)
    }

    pub fn create(&mut self, title: String) -> Todo {
        self.repo.save(title)
    }

    pub fn update(
        &mut self,
        id: u32,
        title: String,
        done: bool,
    ) -> Option<Todo> {
        self.repo.update(id, title, done)
    }
}
