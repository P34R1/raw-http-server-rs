use std::{
    io::prelude::{Read, Write},
    net::TcpStream,
};

pub struct Route {
    http_request_line: String,
    content: Vec<u8>,
}

const OK_STATUS: &str = "HTTP/1.1 200 OK";
const NOT_FOUND_STATUS: &str = "HTTP/1.1 404 NOT FOUND";

const HTML: &str = concat!("Content-Type:", "text/html");
const CSS: &str = concat!("Content-Type:", "text/css");
const FAVICON: &str = concat!("Content-Type:", "image/x-icon");

mod routing;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let _ = Read::read(&mut stream, &mut buf).expect("read the tcp stream");

    let Route {
        http_request_line,
        content,
    } = routing::get_route(&buf);

    let response = [http_request_line.as_bytes(), &content].concat();

    Write::write_all(&mut stream, &response).expect("write to tcp stream");
    TcpStream::flush(&mut stream).expect("flush the tcp stream");
}

#[macro_export]
macro_rules! render_template {
    ($template:expr) => {
        $template.render().expect("get valid html").into_bytes()
    };
}

#[macro_export]
macro_rules! include_static {
    ($file:expr) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $file)).to_vec()
    };
}
