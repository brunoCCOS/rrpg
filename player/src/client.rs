use crate::cli::parser::CommandParseError;

// client.rs
use super::cli;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf};
use std::error::Error;
use tokio::io::{BufReader,BufWriter};
use tokio::io::AsyncBufReadExt;

#[tokio::main]
pub async fn stablish_connection(addr: &str) -> Result<TcpStream, Box<dyn Error>> {
    let stream = TcpStream::connect(addr).await?;
    println!("Connection established {}", addr);
    Ok(stream)
}

pub async fn read_command_line(reader_stdin:&mut BufReader<io::Stdin>, cli:&mut cli::Cli) -> Result<String,Box<dyn Error>> {
    let mut input = String::new();
    reader_stdin.read_line(&mut input).await?;
    match cli.execute(&input) {
        Ok(_) => Ok(input),
        Err(err) => Err(Box::new(err)),
    }
}
