#![feature(const_trait_impl)]

use askama::Template;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

mod connection;

const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 80;

const ADDRESS: SocketAddrV4 = SocketAddrV4::new(IP, PORT);

fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("bind tcplistener to address");

    listener
        .incoming()
        .map(|stream| stream.expect("get tcp stream"))
        .for_each(connection::handle_connection)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    text: &'a str,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate;

const INDEX_TEMPLATE: IndexTemplate = IndexTemplate {
    title: "Anti Afk",
    text: "Anti Afk Utility!",
};

const NOT_FOUND_TEMPLATE: NotFoundTemplate = NotFoundTemplate;
