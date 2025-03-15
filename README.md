# CLEAN

### Exercise: User Management System

An application with the following layers:  
- Entities (Domain Layer) → Pure business logic, independent of frameworks.   
- Use Cases (Application Layer) → Orchestrates business logic and defines application behavior.   
- Adapters (Interface Layer) → Controllers, presenters, and UI elements.   
- Infrastructure (Data Layer) → External services (database, APIs, frameworks).   

```
./
│   ├── Cargo.toml
│   ├── LICENSE
│   ├── Cargo.lock
│   ├── README.md
│   ├── .gitignore
│   ├── src
│   │   ├── main.rs                               # Entry point
│   │   ├── use_cases                             # Application logic
│   │   │   ├── mod.rs
│   │   │   ├── user_service.rs
│   │   ├── infrastructure                        # External services (DB, API)
│   │   │   ├── mod.rs
│   │   │   ├── in_memory_user_repository.rs
│   │   ├── entities                              # Domain models
│   │   │   ├── user.rs
│   │   │   ├── mod.rs
│   │   ├── interfaces                            # Adapters (Repository, Controllers)
│   │   │   ├── mod.rs
│   │   │   ├── user_repository.rs
```

### Run

```bash
cargo run
```