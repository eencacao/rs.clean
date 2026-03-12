use tiny_http::Request;
use serde_json::json;
use crate::usecases::todo_usecase::TodoUseCase;
use crate::infrastructure::memory_repo::MemoryRepo;
use super::handler::{json_response, error_response, read_body};

type Uc = TodoUseCase<MemoryRepo>;

pub fn handle_get_all(uc: &Uc, req: Request) {
    let todos = uc.get_all();
    let body = serde_json::to_string(&todos).unwrap_or("[]".into());
    req.respond(json_response(200, body)).ok();
}

pub fn handle_post(uc: &mut Uc, mut req: Request) {
    let body = read_body(&mut req);
    let v: serde_json::Value =
        serde_json::from_str(&body).unwrap_or(json!({}));
    let title = v["title"].as_str().unwrap_or("").to_string();
    if title.is_empty() {
        req.respond(error_response(400, "title required")).ok();
        return;
    }
    let todo = uc.create(title);
    let out = serde_json::to_string(&todo).unwrap();
    req.respond(json_response(201, out)).ok();
}
