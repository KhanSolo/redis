use crate::resp::RESP;
use crate::server_result::{ServerError, ServerValue};
use crate::set::parse_set_arguments;
use crate::{request::Request, server::Server};

pub async fn command(server: &mut Server, request: &Request, command: &Vec<String>) {
    // validate storage
    //                          &mut Option<Storage> -> Option<&mut Storage>
    let storage = match server.storage.as_mut() {
        Some(s) => s,
        None => {
            request.error(ServerError::StorageNotInitialised).await;
            return;
        }
    };

    // validate command
    if command.len() < 3 {
        request.error(ServerError::CommandSyntaxError(command.join(" "))).await;
        return;
    }

    // params
    let key = command[1].clone();
    let value = command[2].clone();
    let args = match parse_set_arguments(&command[3..].to_vec()) {
        Ok(args) => args,
        Err(_) => {
            request
                .error(ServerError::CommandSyntaxError(command.join(" ")))
                .await;
            return;
        }
    };

    // set
    if let Err(_) = storage.set(key, value, args) {
        request
            .error(ServerError::CommandInternalError(command.join(" ")))
            .await;
        return;
    }

    request
        .data(ServerValue::RESP(RESP::SimpleString(String::from("OK"))))
        .await;
}
