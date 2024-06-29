use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Telling(String),
    Combat(String),
}

pub enum CommandParseError {
    InvalidCommand,
    InvalidArguments,
}

// Implement the Display trait for CommandParseError
impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandParseError::InvalidCommand => write!(f, "Invalid command"),
            CommandParseError::InvalidArguments => write!(f, "Invalid arguments"),
        }
    }
}

// Implement the Debug trait for CommandParseError (optional, but good practice)
impl fmt::Debug for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandParseError::InvalidCommand => write!(f, "CommandParseError::InvalidCommand"),
            CommandParseError::InvalidArguments => write!(f, "CommandParseError::InvalidArguments"),
        }
    }
}

// Implement the std::error::Error trait for CommandParseError
impl std::error::Error for CommandParseError {}


impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Err(CommandParseError::InvalidCommand);
        }

        match parts[0].to_lowercase().as_str() {
            "attack" => {
                if parts.len() != 2 {
                    println!("{:?}, {}", parts[0].to_lowercase().as_str(), parts.len());
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Combat(input.to_owned()))
            }
            "useitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Combat(input.to_owned()))
            }
            "flee" => {
                if parts.len() != 1 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Combat(input.to_owned()))
            }
            "equipitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(input.to_owned()))
            }
            "unequipitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(input.to_owned()))
            }
            "equipmove" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(input.to_owned()))
            }
            "unequipmove" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(input.to_owned()))
            }
            "spendlevelpoints" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                let points = parts[1].parse::<u32>().map_err(|_| CommandParseError::InvalidArguments)?;
                Ok(Command::Telling(input.to_owned()))
            }
            "dropitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(input.to_owned()))
            }
            "pickupitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(input.to_owned()))
            }
            _ => {Err(CommandParseError::InvalidCommand)},
        }
    }
}