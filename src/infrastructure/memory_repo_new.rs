use std::collections::HashMap;
use super::memory_repo::MemoryRepo;

impl MemoryRepo {
    pub fn new() -> Self {
        Self { todos: HashMap::new(), counter: 0 }
    }
}
