use crate::storage_result::{StorageError, StorageResult};

#[derive(Debug, PartialEq)]
pub enum KeyExistence {
    NX,
    XX,
}

#[derive(Debug, PartialEq)]
pub enum KeyExpiry {
    EX(u64),
    PX(u64),
}

#[derive(Debug, PartialEq)]
pub struct SetArgs {
    pub expiry: Option<KeyExpiry>,
    pub existence: Option<KeyExistence>,
    pub get: bool,
}

impl SetArgs {
    pub fn new() -> Self {
        SetArgs {
            expiry: None,
            existence: None,
            get: false,
        }
    }
}

pub fn parse_set_arguments(arguments: &Vec<String>) -> StorageResult<SetArgs> {
    let mut args = SetArgs::new();
    let mut idx: usize = 0;
    loop {
        if idx >= arguments.len() {
            break;
        }

        match arguments[idx].to_lowercase().as_str() {
            "nx" => {
                if args.existence == Some(KeyExistence::XX) {
                    return Err(StorageError::CommandSyntaxError(arguments.join(" ")));
                }
                args.existence = Some(KeyExistence::NX);
                idx += 1;
            }

            _ => {
                return Err(StorageError::CommandSyntaxError(arguments.join(" ")));
            }
        }
    }
    Ok(args)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_nx() {
        let commands: Vec<String> = vec![String::from("NX")];
        let args = parse_set_arguments(&commands).unwrap();
        assert_eq!(args.existence, Some(KeyExistence::NX));
    }
    
    #[test]
    fn test_parse_nx_lowercase() {
        let commands: Vec<String> = vec![String::from("nx")];
        let args = parse_set_arguments(&commands).unwrap();
        assert_eq!(args.existence, Some(KeyExistence::NX));
    }
}
