mod elements;
mod objects;
mod character;
mod cli;
use cli::Cli;
use std::io::stdin;
fn main() {
    let mut cli = Cli::new();
    loop  {
        let mut cmd = String::new();
        stdin().read_line(&mut cmd).expect("Fail to read command");

        // Remove the trailing newline character
        let cmd = cmd.trim();

        cli.execute(cmd);
    }
}
