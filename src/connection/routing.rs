use crate::{INDEX_TEMPLATE, NOT_FOUND_TEMPLATE};
use askama::Template;

const OK_STATUS: &str = "200 OK";
const NOT_FOUND_STATUS: &str = "404 NOT FOUND";

const JAVASCRIPT_MIME_TYPE: &str = concat!("200 OK", "\r\nContent-Type:", "text/javascript");
const CSS_MIME_TYPE: &str = concat!("200 OK", "\r\nContent-Type: ", "text/css");
const FAVICON_MIME_TYPE: &str = concat!("200 OK", "\r\nContent-Type: ", "image/x-icon");

macro_rules! render_template {
    ($template:expr) => {
        $template.render().expect("get valid html").into_bytes()
    };
}

macro_rules! include_static {
    ($file:expr) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $file)).to_vec()
    };
}

macro_rules! define_route {
    ($($name:ident $method:ident $path:literal)+) => {
        #[derive(Clone, Copy)]
        enum Routes {
            $($name,)+
        }

        const ROUTES: &[(Routes, &str)] = &[
            $((
                Routes::$name,
                concat!(stringify!($method), " ", $path, " HTTP/1.1\r\n"),
            ),)+
        ];
    };
}

define_route! {
    Index      GET  "/"
    Htmx       GET  "/htmx.min.js"
    Favicon    GET  "/favicon.ico"
    Stylesheet GET  "/style.css"
}

trait GetRoute {
    fn current_route(&self) -> Option<Routes>;
}
impl GetRoute for &[u8] {
    fn current_route(&self) -> Option<Routes> {
        for (name, http_request_line) in ROUTES {
            if self.starts_with(http_request_line.as_bytes()) {
                return Some(*name);
            }
        }

        None
    }
}

pub struct Route<'a> {
    pub status: &'a str,
    pub content: Vec<u8>,
}

pub fn get_route<'a>(tcp: &[u8]) -> Route<'a> {
    let route = match tcp.current_route() {
        Some(route) => route,

        None => {
            return Route {
                status: NOT_FOUND_STATUS,
                content: render_template!(NOT_FOUND_TEMPLATE),
            };
        }
    };

    let (status, content) = match route {
        Routes::Index => (OK_STATUS, render_template!(INDEX_TEMPLATE)),
        Routes::Htmx => (JAVASCRIPT_MIME_TYPE, include_static!("htmx.min.js")),
        Routes::Favicon => (FAVICON_MIME_TYPE, include_static!("favicon.ico")),
        Routes::Stylesheet => (CSS_MIME_TYPE, include_static!("style.css")),
    };

    Route { status, content }
}
