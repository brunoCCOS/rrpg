pub mod states;
pub mod logging;
pub mod roll_dice;
pub mod parser;

use parser::Command;
use states::{telling::TellingState, State};
use logging::LogLevel::{Info,Warning,Debug,Error,Critical};
pub struct Cli {
    state: states::State,
    logger: logging::Logger
}

impl Cli {
    pub fn new() -> Cli {
        Cli { state: State::Telling(TellingState::new()),
            logger: logging::Logger::new() }
    }

    fn handle_command(&mut self, cmd: Command) {
        match &self.state {
            State::Combat(state) => {
                match cmd {
                    Command::Combat(combat_cmd) => {
                        // Call the execute method on combat state
                        state.execute_command(combat_cmd);
                    }
                    _ => {
                        // Command is not a CombatCommand
                        self.logger.log(Error, "Command is not valid in combat stage");
                    }
                }
            }
            State::Telling(state) => {
                match cmd {
                    Command::Telling(telling_cmd) => {
                        // Call the execute method on telling state
                        state.execute_command(telling_cmd);
                    }
                    _ => {
                        // Command is not a TellingCommand
                        self.logger.log(Error, "Command is not valid outside combat");
                    }
                }
            }
        }
    }

    pub fn execute(&mut self, input: &str) {
        match input.parse::<Command>()  {
            Ok(command) =>self.handle_command(command),
            Err(err) => self.logger.log(Error, &format!("parsing command '{}': {:?}", input, err))
        }
    }
}