use super::states::{combat::CombatCommands,telling::TellingCommands};
use std::str::FromStr;

pub enum Command {
    Telling(TellingCommands),
    Combat(CombatCommands),
}

#[derive(Debug)]
pub enum CommandParseError {
    InvalidCommand,
    InvalidArguments,
}

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
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Combat(CombatCommands::Attack { target: parts[1].to_string() }))
            }
            "move" => {
                if parts.len() != 3 {
                    return Err(CommandParseError::InvalidArguments);
                }
                let x = parts[1].parse::<i32>().map_err(|_| CommandParseError::InvalidArguments)?;
                let y = parts[2].parse::<i32>().map_err(|_| CommandParseError::InvalidArguments)?;
                Ok(Command::Combat(CombatCommands::Move { x, y }))
            }
            "useitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Combat(CombatCommands::UseItem { item_name: parts[1].to_string() }))
            }
            "flee" => {
                if parts.len() != 1 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Combat(CombatCommands::Flee))
            }
            "equipitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(TellingCommands::EquipItem { item_name: parts[1].to_string() }))
            }
            "unequipitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(TellingCommands::UnequipItem { item_name: parts[1].to_string() }))
            }
            "equipmove" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(TellingCommands::EquipMove { spell_name: parts[1].to_string() }))
            }
            "unequipmove" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(TellingCommands::UnequipMove { spell_name: parts[1].to_string() }))
            }
            "spendlevelpoints" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                let points = parts[1].parse::<u32>().map_err(|_| CommandParseError::InvalidArguments)?;
                Ok(Command::Telling(TellingCommands::SpendLevelPoints { points }))
            }
            "dropitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(TellingCommands::DropItem { item_name: parts[1].to_string() }))
            }
            "pickupitem" => {
                if parts.len() != 2 {
                    return Err(CommandParseError::InvalidArguments);
                }
                Ok(Command::Telling(TellingCommands::PickupItem { item_name: parts[1].to_string() }))
            }
            _ => Err(CommandParseError::InvalidCommand),
        }
    }
}