pub mod states;
pub mod parser;

use parser::{Command, CommandParseError};
use states::State;
pub struct Cli {
    state: states::State,
}

impl Cli {
    pub fn new() -> Cli {
        Cli { state: State::Telling }
    }

    fn handle_command(&mut self, cmd: Command,) -> Result<(), CommandParseError> {
        match &self.state {
            State::Combat => {
                match cmd {
                    Command::Combat(_) => {
                        // Return Ok for valid command
                        Ok(())
                    }
                    _ => {
                        // Command is not a CombatCommand
                        Err(CommandParseError::InvalidCommand)
                    }
                }
            }
            State::Telling => {
                match cmd {
                    Command::Telling(_) => {
                        // Return Ok for valid command
                        Ok(())
                    }
                    _ => {
                        // Command is not a TellingCommand
                        Err(CommandParseError::InvalidCommand)
                    }
                }
            }
        }
    }

    pub fn execute(&mut self, input: &str) -> Result<(),CommandParseError> {
        let command =  input.parse::<Command>()?;
        self.handle_command(command)
    }
}