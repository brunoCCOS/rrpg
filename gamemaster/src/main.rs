use tokio::{io::BufReader, net::TcpListener};

pub mod server;
pub mod elements;
pub mod objects;
pub mod cli;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;
    let mut reader_stdin = BufReader::new(tokio::io::stdin());
    let mut cli = cli::Cli::new();
    println!("Server running on {}", addr);
    // Commandline interface
    tokio::spawn(async move {
        server::read_command_line(&mut reader_stdin, &mut cli).await;
    });
    // Loop to accept incoming connections
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("New connection from {}", addr);
                // Spawn a new task for each client connection
                tokio::spawn(async move {
                    server::handle_client(socket).await;
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {:?}", e);
            }
        }
    }


}
