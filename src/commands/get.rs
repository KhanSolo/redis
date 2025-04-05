use crate::resp::RESP;
use crate::server_result::{ServerError, ServerValue};
use crate::{request::Request, server::Server};

pub async fn command(server: &mut Server, request: &Request, command: &Vec<String>) {
    // validate storage
    let storage = match server.storage.as_mut() {
        // &mut Option<Storage> -> Option<&mut Storage>
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
        return; //?
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
    use tokio::sync::mpsc;
}
