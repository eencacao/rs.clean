use crate::interfaces::todo_repository::TodoRepository;
use super::todo_usecase::TodoUseCase;

impl<R: TodoRepository> TodoUseCase<R> {
    pub fn delete(&mut self, id: u32) -> bool {
        self.repo.delete(id)
    }
}
