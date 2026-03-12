use rs_clean::infrastructure::memory_repo::MemoryRepo;
use rs_clean::usecases::todo_usecase::TodoUseCase;

#[test]
fn test_delete() {
    let mut uc = TodoUseCase::new(MemoryRepo::new());
    uc.create("bye".to_string());
    assert!(uc.delete(1));
    assert!(!uc.delete(1));
}
