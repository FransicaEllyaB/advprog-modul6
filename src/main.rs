use std::{
    fs,
    io::{prelude::*,BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
 
fn handle_connection(mut stream: TcpStream) { 
    let buf_reader = BufReader::new(&mut stream); 
    let request_line = buf_reader.lines().next().unwrap().unwrap();
 
    let (status_line, filename);
    if request_line == "GET / HTTP/1.1" {
        status_line = "HTTP/1.1 200 OK";
        filename = "hello.html";
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        filename = "404.html";
    }

    let response = build_response(status_line, filename);
    stream.write_all(response.as_bytes()).unwrap(); 
} 

fn build_response(status_line: &str, filename: &str) -> String {
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    )
}