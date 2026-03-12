use std::collections::HashMap;
use crate::entities::todo::Todo;
use crate::interfaces::todo_repository::TodoRepository;

pub struct MemoryRepo {
    pub todos: HashMap<u32, Todo>,
    pub counter: u32,
}

impl TodoRepository for MemoryRepo {
    fn get_all(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect()
    }

    fn get_by_id(&self, id: u32) -> Option<Todo> {
        self.todos.get(&id).cloned()
    }

    fn save(&mut self, title: String) -> Todo {
        self.counter += 1;
        let todo = Todo::new(self.counter, title);
        self.todos.insert(self.counter, todo.clone());
        todo
    }

    fn update(
        &mut self,
        id: u32,
        title: String,
        done: bool,
    ) -> Option<Todo> {
        let todo = self.todos.get_mut(&id)?;
        todo.title = title;
        todo.completed = done;
        Some(todo.clone())
    }

    fn delete(&mut self, id: u32) -> bool {
        self.todos.remove(&id).is_some()
    }
}
