use tiny_http::Request;
use serde_json::json;
use crate::usecases::todo_usecase::TodoUseCase;
use crate::infrastructure::memory_repo::MemoryRepo;
use super::handler::{json_response, error_response, read_body};

type Uc = TodoUseCase<MemoryRepo>;

pub fn handle_get_one(uc: &Uc, req: Request, id: u32) {
    match uc.get_by_id(id) {
        Some(t) => {
            let out = serde_json::to_string(&t).unwrap();
            req.respond(json_response(200, out)).ok();
        }
        None => {
            req.respond(error_response(404, "not found")).ok();
        }
    }
}

pub fn handle_put(uc: &mut Uc, mut req: Request, id: u32) {
    let body = read_body(&mut req);
    let v: serde_json::Value =
        serde_json::from_str(&body).unwrap_or(json!({}));
    let title = v["title"].as_str().unwrap_or("").to_string();
    let done = v["completed"].as_bool().unwrap_or(false);
    match uc.update(id, title, done) {
        Some(t) => {
            let out = serde_json::to_string(&t).unwrap();
            req.respond(json_response(200, out)).ok();
        }
        None => {
            req.respond(error_response(404, "not found")).ok();
        }
    }
}

pub fn handle_delete(uc: &mut Uc, req: Request, id: u32) {
    if uc.delete(id) {
        req.respond(json_response(204, "".into())).ok();
    } else {
        req.respond(error_response(404, "not found")).ok();
    }
}
