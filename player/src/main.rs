// client.rs
pub mod cli;
pub mod client;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut cli = cli::Cli::new();
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = stream.split();

    let mut reader: BufReader<tokio::net::tcp::ReadHalf> = BufReader::new(reader);
    let mut reader_stdin = BufReader::new(tokio::io::stdin());
    let mut input ;
    let mut buf: Vec<u8> = vec![0; 1024];

    loop {
        input = client::read_command_line(&mut reader_stdin, &mut cli).await;

        match input {
            Ok(result) => {
                writer.write_all(result.as_bytes()).await?;

                let n = reader.read(&mut buf).await?;
                if n == 0 {
                    println!("Server disconnected");
                    break;
                }

                println!("Received from server: {}", String::from_utf8_lossy(&buf[..n]));
            }
            Err(e) => {
                println!("Error: {}", e);
                continue; // Skip to the next iteration
            }
        }
    }

    Ok(())
}
