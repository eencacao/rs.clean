use rs_clean::infrastructure::memory_repo::MemoryRepo;
use rs_clean::usecases::todo_usecase::TodoUseCase;

fn make_uc() -> TodoUseCase<MemoryRepo> {
    TodoUseCase::new(MemoryRepo::new())
}

#[test]
fn test_create() {
    let mut uc = make_uc();
    let todo = uc.create("buy milk".to_string());
    assert_eq!(todo.id, 1);
    assert_eq!(todo.title, "buy milk");
    assert!(!todo.completed);
    assert!(!todo.created_at.is_empty());
}

#[test]
fn test_get_all() {
    let mut uc = make_uc();
    assert!(uc.get_all().is_empty());
    uc.create("a".to_string());
    uc.create("b".to_string());
    assert_eq!(uc.get_all().len(), 2);
}

#[test]
fn test_get_by_id() {
    let mut uc = make_uc();
    uc.create("find me".to_string());
    assert!(uc.get_by_id(1).is_some());
    assert_eq!(uc.get_by_id(1).unwrap().title, "find me");
    assert!(uc.get_by_id(99).is_none());
}

#[test]
fn test_update() {
    let mut uc = make_uc();
    uc.create("old".to_string());
    let r = uc.update(1, "new".to_string(), true);
    assert!(r.is_some());
    assert_eq!(r.unwrap().title, "new");
    assert!(uc.update(99, "x".to_string(), false).is_none());
}
