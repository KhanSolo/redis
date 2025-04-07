use crate::resp::RESP;
use crate::server_result::{ServerError, ServerValue};
use crate::{request::Request, server::Server};

pub async fn command(server: &mut Server, request: &Request, command: &Vec<String>) {
    // validate storage
    //                           &mut Option<Storage> -> Option<&mut Storage>
    let storage = match server.storage.as_mut() {
        Some(s) => s,
        None => {
            request.error(ServerError::StorageNotInitialised).await;
            return;
        }
    };

    // validate command
    if command.len() != 2 {
        request
            .error(ServerError::CommandSyntaxError(command.join(" ")))
            .await;
        return;
    }

    // get
    let output = storage.get(command[1].clone());
    match output {
        Ok(Some(v)) => request.data(ServerValue::RESP(RESP::BulkString(v))).await,
        Ok(None) => request.data(ServerValue::RESP(RESP::Null)).await,
        Err(_) => {
            request
                .error(ServerError::CommandInternalError(command.join(" ")))
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server_result::ServerMessage;
    use crate::set::SetArgs;
    use crate::storage::Storage;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_command() {
        let mut storage = Storage::new();
        storage
            .set(String::from("key"), String::from("value"), SetArgs::new())
            .unwrap();
        let mut server = Server::new().set_storage(storage);
        let cmd = vec![String::from("get"), String::from("key")];
        let (sender, mut receiver) = mpsc::channel::<ServerMessage>(32);

        let request = Request {
            value: RESP::Null,
            sender: sender,
        };

        command(&mut server, &request, &cmd).await;

        assert_eq!(
            receiver.try_recv().unwrap(),
            ServerMessage::Data(ServerValue::RESP(RESP::BulkString(String::from("value"))))
        );
    }

    #[tokio::test]
    async fn test_storage_not_initialised() {
        let mut server = Server::new();
        let cmd = vec![String::from("get"), String::from("key")];
        let (sender, mut receiver) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::Null,
            sender: sender,
        };

        command(&mut server, &request, &cmd).await;

        assert_eq!(
            receiver.try_recv().unwrap(),
            ServerMessage::Error(ServerError::StorageNotInitialised)
        );
    }

    #[tokio::test]
    async fn test_wrong_syntax() {
        let storage = Storage::new();
        let mut server = Server::new().set_storage(storage);
        let cmd = vec![String::from("get")];
        let (sender, mut receiver) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::Null,
            sender: sender,
        };

        command(&mut server, &request, &cmd).await;

        assert_eq!(
            receiver.try_recv().unwrap(),
            ServerMessage::Error(ServerError::CommandSyntaxError("get".to_string()))
        );
    }
}
