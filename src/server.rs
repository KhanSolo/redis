use crate::commands::{echo, get, ping, set};
use crate::connection::ConnectionMessage;
use crate::request::Request;
use crate::server_result::{ServerError, ServerValue};
use crate::{
    storage::Storage,
    storage_result::{StorageError, StorageResult},
    RESP,
};
use std::time::Duration;
// use std::{
//     fmt,
//     sync::{Arc, Mutex},
// };
use tokio::sync::mpsc;

pub struct Server {
    pub storage: Option<Storage>,
}

impl Server {
    pub fn new() -> Self {
        Self { storage: None }
    }

    pub fn set_storage(mut self, storage: Storage) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn expire_keys(&mut self) {
        let storage = match self.storage.as_mut() {
            Some(storage) => storage,
            None => return,
        };
        storage.expire_keys();
    }
}

pub async fn process_request(request: Request, server: &mut Server) {
    let elements = match &request.value {
        RESP::Array(v) => v,
        _ => {
            request.error(ServerError::IncorrectData).await;
            return;
        }
    };

    let mut command = Vec::new();
    for elem in elements.iter() {
        match elem {
            // the vector command now needs a clone of the bulk string because we are eventually
            //transferring its ownership to storage.process_command
            RESP::BulkString(v) => command.push(v.clone()),
            _ => {
                request.error(ServerError::IncorrectData).await;
                return;
            }
        }
    }

    let command_name = command[0].to_lowercase();
    match command_name.as_str() {
        "echo" => {
            echo::command(server, &request, &command).await;
        }
        "get" => {
            get::command(server, &request, &command).await;
        }
        "ping" => {
            ping::command(server, &request, &command).await;
        }
        "set" => {
            set::command(server, &request, &command).await;
        }
        _ => {
            request
                .error(ServerError::CommandNotAvailable(command[0].clone()))
                .await;
        }
    }
}

pub async fn run_server(mut server: Server, mut crx: mpsc::Receiver<ConnectionMessage>) {
    let mut interval_timer = tokio::time::interval(Duration::from_millis(10));
    loop {
        tokio::select! {
            Some(message) = crx.recv() => {
                match message {
                    ConnectionMessage::Request(request) => {
                        process_request(request, &mut server).await;
                    }
                }
            }

            _ = interval_timer.tick() => {
                server.expire_keys();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::server_result::ServerMessage;
    use tokio::sync::mpsc;

    use super::*;

    #[tokio::test]
    async fn test_process_request_not_array() {
        let (connection_sender, mut connection_receiver) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::BulkString(String::from("PING")),
            sender: connection_sender,
        };

        let storage = Storage::new();
        let mut server = &mut Server::new().set_storage(storage);
        process_request(request, &mut server).await;
        assert_eq!(
            connection_receiver.try_recv().unwrap(),
            ServerMessage::Error(ServerError::IncorrectData)
        );
    }

    #[tokio::test]
    async fn test_process_request_not_bulkstrings() {
        let (connection_sender, mut connection_receiver) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::Array(vec![RESP::SimpleString(String::from("PING"))]),
            sender: connection_sender,
        };

        let storage = Storage::new();
        let mut server = &mut Server::new().set_storage(storage);
        process_request(request, &mut server).await;
        assert_eq!(
            connection_receiver.try_recv().unwrap(),
            ServerMessage::Error(ServerError::IncorrectData)
        );
    }

    #[tokio::test]
    async fn test_process_request_echo() {
        let (connection_sender, mut connection_receiver) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::Array(vec![
                RESP::BulkString(String::from("ECHO")),
                RESP::BulkString(String::from("42")),
            ]),
            sender: connection_sender,
        };

        let storage = Storage::new();
        let mut server = &mut Server::new().set_storage(storage);

        process_request(request, &mut server).await;
        assert_eq!(
            connection_receiver.try_recv().unwrap(),
            ServerMessage::Data(ServerValue::RESP(RESP::BulkString(String::from("42"))))
        );
    }

    #[test]
    fn test_create_new() {
        let server: Server = Server::new();
        match server.storage {
            Some(_) => panic!(),
            None => (),
        };
    }

    #[test]
    fn test_set_storage() {
        let storage = Storage::new();
        let server: Server = Server::new().set_storage(storage);
        match server.storage {
            Some(_) => (),
            None => panic!(),
        };
    }
}
