pub mod states;
pub mod logging;
pub mod roll_dice;


use clap::{Parser};

#[derive(Parser)]
#[command(name = "RPG CLI", version = "1.0", author = "Your Name", about = "A tabletop RPG command line interface")]
pub struct Cli {
    state: states::State,
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn execute(&self) {
    }
}
