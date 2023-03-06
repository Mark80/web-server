use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let socket = TcpListener::bind("localhost:8080").unwrap();

    for stream in socket.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    };
}

fn handle_connection(mut connection: TcpStream) {
    let buffer_reader = BufReader::new(&mut connection);
    let http_request: Vec<_> = buffer_reader
        .lines()
        .map(|r| r.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    connection.write_all(response.as_bytes()).unwrap();

    println!("Request: {:#?}", http_request);
}
