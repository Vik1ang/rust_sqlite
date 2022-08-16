use std::fmt;
use std::fmt::{Formatter, Write};

#[derive(Debug, PartialEq)]
pub enum MetaCommand {
    Exit,
    Help,
    Open(String),
    Unknown,
}

impl fmt::Display for MetaCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MetaCommand::Exit => f.write_str(".exit"),
            MetaCommand::Help => f.write_str(".help"),
            MetaCommand::Open(_) => f.write_str(".open"),
            MetaCommand::Unknown => f.write_str("Unknown command"),
        }
    }
}

impl MetaCommand {
    pub fn new(command: String) -> MetaCommand {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();
        match cmd.as_ref() {
            ".exit" => MetaCommand::Exit,
            ".help" => MetaCommand::Help,
            ".open" => MetaCommand::Open(command),
            _ => MetaCommand::Unknown,
        }
    }
}