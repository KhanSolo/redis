use crate::request::Request;
//use crate::server_result::ServerMessage;
use crate::{
    storage::Storage,
    storage_result::{StorageError, StorageResult},
    RESP,
};
use std::{
    fmt,
    sync::{Arc, Mutex},
};

#[derive(Debug, PartialEq)]
pub enum ServerError {
    CommandError,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::CommandError => write!(f, "Error while processing!"),
        }
    }
}

pub type ServerResult<T> = Result<T, ServerError>;

pub fn process_request(request: Request, storage: Arc<Mutex<Storage>>) -> StorageResult<RESP> {
    let elements = match request.value {
        RESP::Array(v) => v,
        _ => {
            return Err(StorageError::IncorrectRequest);
        }
    };

    let mut command = Vec::new();
    for elem in elements.iter() {
        match elem {
            // the vector command now needs a clone of the bulk string because we are eventually
            //transferring its ownership to storage.process_command
            RESP::BulkString(v) => command.push(v.clone()),
            _ => {
                return Err(StorageError::IncorrectRequest);
            }
        }
    }

    let mut guard = storage.lock().unwrap();
    let response = guard.process_command(&command);
    response
}

#[cfg(test)]
mod tests {
    use crate::server_result::ServerMessage;
    use tokio::sync::mpsc;

    use super::*;

    #[test]
    fn test_process_request_ping() {
        let (connection_sender, _) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::Array(vec![RESP::BulkString(String::from("PING"))]),
            sender: connection_sender,
        };

        let storage = Arc::new(Mutex::new(Storage::new()));
        let output = process_request(request, storage).unwrap();
        assert_eq!(output, RESP::SimpleString(String::from("PONG")));
    }

    #[test]
    fn test_process_request_not_array() {
        let (connection_sender, _) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::BulkString(String::from("PING")),
            sender: connection_sender,
        };

        let storage = Arc::new(Mutex::new(Storage::new()));
        let error = process_request(request, storage).unwrap_err();
        assert_eq!(error, StorageError::IncorrectRequest);
    }

    #[test]
    fn test_process_request_not_bulkstrings() {
        let (connection_sender, _) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::Array(vec![RESP::SimpleString(String::from("PING"))]),
            sender: connection_sender,
        };

        let storage = Arc::new(Mutex::new(Storage::new()));
        let error = process_request(request, storage).unwrap_err();
        assert_eq!(error, StorageError::IncorrectRequest);
    }

    #[test]
    fn test_process_request_echo() {
        let (connection_sender, _) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::Array(vec![
                RESP::BulkString(String::from("ECHO")),
                RESP::BulkString(String::from("42")),
            ]),
            sender: connection_sender,
        };
        let storage = Arc::new(Mutex::new(Storage::new()));
        let output = process_request(request, storage).unwrap();
        assert_eq!(output, RESP::BulkString(String::from("42")));
    }
}
