use crate::Command::{Publish, Retrieve};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteMessage,
    UnexpectedNewline,
    EmptyMessage,
    UnknownCommand,
    UnexpectedPayload,
    MissingPayload,
}

pub fn parse(line: &str) -> Result<Command, Error> {
    let Some(message) = line.strip_suffix('\n') else {
        return Err(Error::IncompleteMessage); // a message must end with '\n'
    };

    if message.contains('\n') {
        return Err(Error::UnexpectedNewline); // a message can only contain a single '\n'
    };

    if message.starts_with("PUBLISH") {
        if let Some(payload) = message.strip_prefix("PUBLISH ") {
            Ok(Publish(payload.into()))
        } else {
            Err(Error::MissingPayload)
        }
    } else if message.starts_with("RETRIEVE") {
        if let Some(_payload) = message.strip_prefix("RETRIEVE ") {
            Err(Error::UnexpectedPayload)
        } else {
            Ok(Retrieve)
        }
    } else if message.is_empty() {
        Err(Error::EmptyMessage)
    } else {
        Err(Error::UnknownCommand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests placement of \n
    #[test]
    fn test_missing_nl() {
        let line = "RETRIEVE";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_trailing_data() {
        let line = "PUBLISH The message\n is wrong \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedNewline);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_string() {
        let line = "";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }

    // Tests for empty messages and unknown commands

    #[test]
    fn test_only_nl() {
        let line = "\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::EmptyMessage);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unknown_command() {
        let line = "SERVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnknownCommand);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of RETRIEVE command

    #[test]
    fn test_retrieve_w_whitespace() {
        let line = "RETRIEVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve_payload() {
        let line = "RETRIEVE this has a payload\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve() {
        let line = "RETRIEVE\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Retrieve);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of PUBLISH command

    #[test]
    fn test_publish() {
        let line = "PUBLISH TestMessage\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("TestMessage".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_publish() {
        let line = "PUBLISH \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_missing_payload() {
        let line = "PUBLISH\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::MissingPayload);
        assert_eq!(result, expected);
    }
}
