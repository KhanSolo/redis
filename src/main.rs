use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                println!("some bytes were read {}", size);
                let string = str::from_utf8(&buffer).expect("Our bytes should be valid utf8");
                println!("{string}");

                let response = "+PONG\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Ok(_) => {
                println!("connection closed");
                break;
            }
            Err(e) => {
                println!("Error : {e}");
                break;
            }
        }
    }
}
