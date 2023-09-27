use super::routing::Route;

pub fn get_response(Route { status, content }: Route) -> Vec<u8> {
    let http_request_line = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n",
        status,
        content.len(),
    )
    .into_bytes();

    [http_request_line, content].concat()
}
