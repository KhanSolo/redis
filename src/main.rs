use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
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
    println!("accepted new connection");
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    {
        let string = str::from_utf8(&buffer).expect("Our bytes should be valid utf8");
        println!("{string}");
        //println!("{:?}", &buffer);
    }

    let response = "+PONG\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
