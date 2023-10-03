use super::{Route, CSS, FAVICON, HTML, NOT_FOUND_STATUS, OK_STATUS};
use crate::{include_static, render_template, INDEX_TEMPLATE, NOT_FOUND_TEMPLATE};
use askama::Template;

fn http_request_line(status: &str, mime_type: &str, content_length: usize) -> String {
    format!("{status}\r\n{mime_type}\r\nContent-Length:{content_length}\r\n\r\n",)
}

fn not_found_template() -> Route {
    let content = render_template!(NOT_FOUND_TEMPLATE);
    let http_request_line = http_request_line(NOT_FOUND_STATUS, HTML, content.len());

    Route {
        http_request_line,
        content,
    }
}

macro_rules! define_route {
    ($($name:ident $method:ident $path:literal)+) => {
        enum Routes {
            $($name,)+
        }

        fn current_route(tcp: &[u8]) -> Option<Routes> {
            //                   GET /style.css HTTP/1.1\r\n
            $(if tcp.starts_with(concat!(stringify!($method), " ", $path, " HTTP/1.1\r\n").as_bytes()) {
                Some(Routes::$name)
            } else)+ {
                None
            }
        }
    };
}

define_route! {
    Index       GET  "/"
    Favicon     GET  "/favicon.ico"
    Stylesheet  GET  "/style.css"
}

pub fn get_route(tcp: &[u8]) -> Route {
    let route = match current_route(tcp) {
        Some(route) => route,
        None => {
            return not_found_template();
        }
    };

    let (mime_type, content) = match route {
        Routes::Index => (HTML, render_template!(INDEX_TEMPLATE)),
        Routes::Favicon => (FAVICON, include_static!("favicon.ico")),
        Routes::Stylesheet => (CSS, include_static!("style.css")),
    };

    let http_request_line = http_request_line(OK_STATUS, mime_type, content.len());

    Route {
        http_request_line,
        content,
    }
}
