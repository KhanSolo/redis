use crate::{request::Request, server::Server};
use crate::server_result::ServerValue;
use crate::resp::RESP;

pub async fn command(_server:&Server, request:&Request, _command:&Vec<String>){
    request
        .data(ServerValue::RESP(RESP::SimpleString("PONG".to_string())))
        .await;
}

#[cfg(test)]
mod tests{
    use crate::{server, server_result::ServerMessage, storage::Storage};
    use tokio::sync::mpsc;

    use super::*;

    // #[tokio::test]
    // async fn test_process_request_ping() {
    //     let (connection_sender, mut connection_receiver) = mpsc::channel::<ServerMessage>(32);
    //     let request = Request {
    //         value: RESP::Array(vec![RESP::BulkString(String::from("PING"))]),
    //         sender: connection_sender,
    //     };

    //     let storage = Storage::new();
    //     let mut server = &mut Server::new().set_storage(storage);
    //     process_request(request, &mut server).await;

    //     assert_eq!(
    //         connection_receiver.try_recv().unwrap(),
    //         ServerMessage::Data(
    //             ServerValue::RESP(
    //                 RESP::SimpleString(String::from("PONG"))))
    //             );
    // }

    #[tokio::test]
    async fn test_command_ping() {
        let cmd = vec![String::from("ping")];
        let server = Server::new();
        let (connection_sender, mut connection_receiver) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::Null,
            sender: connection_sender,
        };
        command(&server, &request, &cmd).await;
        assert_eq!(
            connection_receiver.try_recv().unwrap(),
            ServerMessage::Data(ServerValue::RESP(RESP::SimpleString(String::from("PONG"))))
        )
    }
    
    // #[test]
    // fn test_command_ping_uppercase() {
    //     let command = vec![String::from("PING")];
    //     let storage: Storage = Storage::new();
    //     let output = storage.command_ping(&command).unwrap();
    //     assert_eq!(output, RESP::SimpleString(String::from("PONG")));
    // }

    #[tokio::test]
    async fn test_command_ping_uppercase() {
        let cmd = vec![String::from("PING")];
        let server = Server::new();
        let (connection_sender, mut connection_receiver) = mpsc::channel::<ServerMessage>(32);
        let request = Request {
            value: RESP::Null,
            sender: connection_sender,
        };
        command(&server, &request, &cmd).await;
        assert_eq!(
            connection_receiver.try_recv().unwrap(),
            ServerMessage::Data(ServerValue::RESP(RESP::SimpleString(String::from("PONG" ))))
        )
    }
}