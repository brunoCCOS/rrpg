use super::cli;
use std::error::Error;
use tokio::io::{BufReader,BufWriter};
use tokio::io::AsyncBufReadExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::BytesMut;
use cli::logging::{Logger, LogLevel};
use LogLevel::{Info, Warning,Critical,Debug};
use tokio::net::{TcpListener, TcpStream};


// Function to handle each client connection
pub async fn handle_client(mut socket: TcpStream) {
    let mut buf = BytesMut::with_capacity(1024);

    loop {
        buf.clear();
        // Read data from the socket
        match socket.read_buf(&mut buf).await {
            Ok(0) => {
                // Connection closed
                println!("Client disconnected");
                return;
            }
            Ok(n) => {
                // Echo the received data back to the client
                if socket.write_all(&buf[..n]).await.is_err() {
                    println!("Failed to send data to client");
                    return;
                }
            }
            Err(e) => {
                println!("Error reading from socket: {:?}", e);
                return;
            }
        }
    }
}



pub async fn read_command_line(reader_stdin:&mut BufReader<tokio::io::Stdin>, cli:&mut cli::Cli)  {
    let mut input = String::new();
    loop {
        input.clear();
        match reader_stdin.read_line(&mut input).await {
            Ok(_) =>  cli.execute(&input),
            Err(e) => {}
        }

    }
}
