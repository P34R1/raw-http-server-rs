use std::{
    io::prelude::{Read, Write},
    net::TcpStream,
};

mod connection_response;
mod routing;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let _ = Read::read(&mut stream, &mut buf).expect("read the tcp stream");

    let route = routing::get_route(&buf);
    let response = connection_response::get_response(route);

    Write::write_all(&mut stream, &response).expect("write to tcp stream");
    TcpStream::flush(&mut stream).expect("flush the tcp stream");
}
