use tiny_http::{Header, Request, Response, StatusCode};
use serde_json::json;

pub fn json_response(
    code: u16,
    body: String,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let header = Header::from_bytes(
        &b"Content-Type"[..],
        &b"application/json"[..],
    )
    .unwrap();
    Response::from_string(body)
        .with_status_code(StatusCode(code))
        .with_header(header)
}

pub fn error_response(
    code: u16,
    msg: &str,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = json!({"error": msg}).to_string();
    json_response(code, body)
}

pub fn parse_id(path: &str) -> Option<u32> {
    path.trim_matches('/').split('/').nth(1)?.parse().ok()
}

pub fn read_body(req: &mut Request) -> String {
    let mut body = String::new();
    req.as_reader().read_to_string(&mut body).unwrap_or(0);
    body
}
