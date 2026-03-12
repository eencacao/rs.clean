# Todo API — Rust Clean Architecture

A RESTful To-Do list API built with Rust, following Clean Architecture
principles with an in-memory database.

## Architecture

```
src/
├── entities/        Pure domain model (Todo)
├── interfaces/      Repository contract
├── usecases/        Business logic (CRUD)
├── infrastructure/  In-memory repository
└── adapters/        HTTP handlers
```

Each layer has a single responsibility:

| Layer | Responsibility |
|---|---|
| `entities/` | Pure domain model (`Todo`) |
| `interfaces/` | Repository contract |
| `usecases/` | Business logic (CRUD) |
| `infrastructure/` | In-memory repository |
| `adapters/` | HTTP handlers |

## API Endpoints

| Method | Endpoint | Description | Status |
|---|---|---|---|
| GET | /todos | List all todos | 200 |
| POST | /todos | Create a todo | 201 |
| GET | /todos/{id} | Get by ID | 200 |
| PUT | /todos/{id} | Update a todo | 200 |
| DELETE | /todos/{id} | Delete a todo | 204 |

## Todo Object

```json
{
  "id": 1,
  "title": "Buy groceries",
  "completed": false,
  "created_at": "2026-03-12T11:00:00Z"
}
```

## Requirements

- Rust 1.70+
- Cargo

## Setup & Run

```bash
cargo run
```

Server runs on `http://localhost:8080`.  
Override port: `PORT=9000 cargo run`

## Testing

```bash
# Unit and integration tests
cargo test

# Functional tests (requires a running server on port 18080)
PORT=18080 cargo run &
bash tests/api_test.sh
```
