mod entities;
mod infrastructure;
mod interfaces;
mod use_cases;

use infrastructure::in_memory_user_repository::InMemoryUserRepository;
use use_cases::user_service::UserService;

fn main() {
    let repo = InMemoryUserRepository::new();
    let mut service = UserService::new(repo);

    service.create_user("Alice".to_string(), "alice@example.com".to_string());
    service.create_user("Bob".to_string(), "bob@example.com".to_string());

    if let Some(user) = service.get_user(1) {
        println!(
            "User found: User {{ id: {}, name: \"{}\", email: \"{}\" }}",
            user.id, user.name, user.email
        );
    } else {
        println!("User not found");
    }
}
