use crate::{request::Request, server::Server};
use crate::server_result::ServerValue;
use crate::resp::RESP;

pub async fn command(_server:&Server, request:&Request, command:&Vec<String>){    
    request
        .data(ServerValue::RESP(RESP::BulkString(command[1].to_string())))
        .await;
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;
    use crate::server_result::ServerMessage;
    use super::*;

    #[tokio::test]
    async fn test_command() {
        let server = Server::new();
        let cmd = vec![String::from("echo"), String::from("hi")];
        let (sender, mut receiver) = mpsc::channel::<ServerMessage>(32);

        let request = Request { value: RESP::Null, sender: sender};

        command(&server, &request, &cmd).await;

        assert_eq!(
            receiver.try_recv().unwrap(),
            ServerMessage::Data(ServerValue::RESP(RESP::BulkString(String::from("hi"))))
        )
    }
}