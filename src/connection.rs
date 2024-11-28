use crate::request::Request;
use crate::resp::bytes_to_resp;
use crate::server_result::{ServerError, ServerMessage, ServerValue};
use core::str;
use std::fmt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    select,
    sync::mpsc,
};

#[derive(Debug)]
pub enum ConnectionError {
    ServerError(ServerError),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::ServerError(e) => {
                write!(f, "{}", format!("Server error: {}", e))
            }
        }
    }
}

#[derive(Debug)]
pub enum ConnectionMessage {
    Request(Request),
}

pub async fn run_listener(host: String, port: u16, server_sender: mpsc::Sender<ConnectionMessage>) {
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    loop {
        tokio::select! {
            connection = listener.accept() => {
                match connection {
                    Ok((stream, _)) => {
                        tokio::spawn(handle_connection(stream, server_sender.clone()));
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        continue;
                    }
                }
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, server_sender: mpsc::Sender<ConnectionMessage>) {
    let mut buffer = [0; 512];

    let (connection_sender, mut connection_receiver) = mpsc::channel::<ServerMessage>(32);

    loop {
        select! {
            result = stream.read(&mut buffer) => {
                match result {
                    Ok(size) if size > 0 => {
                        println!("some bytes were read {}", size);
                        let string = str::from_utf8(&buffer).expect("Our bytes should be valid utf8");
                        println!("{string}");

                        let mut index: usize = 0;
                        let resp = match bytes_to_resp(&buffer[..size].to_vec(), &mut index) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                return;
                            }
                        };

                        let request = Request {
                            value: resp,
                            sender: connection_sender.clone(),
                        };

                        match server_sender.send(ConnectionMessage::Request(request)).await {
                            Ok(()) => {},
                            Err(e) => {
                                eprintln!("Error sending request: {}", e);
                                return;
                            }
                        }
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
            },

            Some(response) = connection_receiver.recv() => {
                    let _ = match response {

                        ServerMessage::Data(ServerValue::RESP(v)) => stream.write_all(v.to_string().as_bytes()).await,

                        ServerMessage::Error(e) => {
                            eprintln!("Error: {}", ConnectionError::ServerError(e));
                            return;
                        }
                    };
                }
        }
    }
}
