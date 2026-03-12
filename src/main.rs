use tiny_http::Server;
use rs_clean::infrastructure::memory_repo::MemoryRepo;
use rs_clean::usecases::todo_usecase::TodoUseCase;
use rs_clean::adapters::{router, item_router, handler};

fn route(uc: &mut TodoUseCase<MemoryRepo>, req: tiny_http::Request) {
    let method = req.method().as_str().to_string();
    let path = req.url().to_string();
    let is_item = path.starts_with("/todos/") && path.len() > 7;
    if !is_item {
        match method.as_str() {
            "GET" => router::handle_get_all(uc, req),
            "POST" => router::handle_post(uc, req),
            _ => {
                req.respond(
                    handler::error_response(405, "not allowed"),
                )
                .ok();
            }
        }
    } else {
        let id = handler::parse_id(&path).unwrap_or(0);
        match method.as_str() {
            "GET" => item_router::handle_get_one(uc, req, id),
            "PUT" => item_router::handle_put(uc, req, id),
            "DELETE" => item_router::handle_delete(uc, req, id),
            _ => {
                req.respond(
                    handler::error_response(405, "not allowed"),
                )
                .ok();
            }
        }
    }
}

fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).unwrap();
    let mut uc = TodoUseCase::new(MemoryRepo::new());
    println!("Listening on :{}", port);
    for req in server.incoming_requests() {
        route(&mut uc, req);
    }
}
