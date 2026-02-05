use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

type Result<T> = std::result::Result<T, anyhow::Error>;

async fn accept(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    // loop to keep accepting new connections to the socket
    loop {
        if let Ok((socket, addr)) = listener.accept().await {
            println!("Got a new connection from {}", addr);
            tokio::spawn(connection_loop(socket));
        }
    }
}

async fn connection_loop(socket: TcpStream) -> Result<()> {
    let mut lines = BufReader::new(socket).lines();

    loop {
        match lines.next_line().await {
            Ok(Some(line)) => println!("received from reader: {line}"),

            Ok(None) => {
                println!("client disconnected");
                break;
            }

            Err(error) => {
                println!("error: {error}");
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
pub(crate) async fn main() -> Result<()> {
    accept("127.0.0.1:8080".to_string()).await
}
