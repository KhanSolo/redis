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
    use crate::server_result::ServerMessage;
    use tokio::sync::mpsc;

    use super::*;

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