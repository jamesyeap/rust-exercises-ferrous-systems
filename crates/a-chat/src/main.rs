use anyhow::bail;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Receiver, Sender, channel};

type Result<T> = std::result::Result<T, anyhow::Error>;

async fn accept(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let app = Arc::new(Mutex::new(Application::new()));

    // loop to keep accepting new connections to the socket
    loop {
        if let Ok((socket, addr)) = listener.accept().await {
            println!("Got a new connection from {}", addr);

            // create a new task, to be scheduled asynchronously by tokio
            //   DOES NOT BLOCK HERE
            tokio::spawn(connection_loop(socket, app.clone()));

            // after the task is created, we're ready to accept new connections
        }
    }
}

struct Application {
    // holds a list of channels that can be used to send messages to users
    user_channels: HashMap<String, Sender<String>>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            user_channels: HashMap::new(),
        }
    }
    pub fn register_user(&mut self, username: &str) -> Result<Receiver<String>> {
        if self.user_channels.contains_key(username) {
            bail!("Username already taken! {username}");
        }

        let (sender, receiver) = channel(100);
        self.user_channels.insert(username.into(), sender);

        Ok(receiver)
    }

    pub fn get_user_channel(&self, username: &str) -> Option<&Sender<String>> {
        self.user_channels.get(username)
    }
}

async fn get_messages_from_other_users(
    writer: Arc<Mutex<BufWriter<OwnedWriteHalf>>>,
    mut incoming_messages: Receiver<String>,
) -> Result<()> {
    loop {
        if let Some(message) = incoming_messages.recv().await {
            let mut writer = writer.lock().await;
            writer.write(message.as_bytes()).await?;
            writer.write("\n".as_bytes()).await?;
            writer.flush().await?;
            drop(writer);
        }
    }
}

async fn connection_loop(socket: TcpStream, app: Arc<Mutex<Application>>) -> Result<()> {
    let (reader, writer) = socket.into_split();

    let mut lines = BufReader::new(reader).lines();
    let writer = Arc::new(Mutex::new(BufWriter::new(writer)));

    // get username
    let username = lines
        .next_line()
        .await?
        .ok_or(anyhow::anyhow!("Must provide username!"))?;

    // register user to receive messages from other users
    let mut guard = app.lock().await; // lock the mutex
    let incoming_messages = guard.register_user(&username)?;
    drop(guard); // unlock the mutex
    tokio::spawn(get_messages_from_other_users(
        writer.clone(),
        incoming_messages,
    ));

    loop {
        match lines.next_line().await {
            Ok(Some(line)) => {
                match line.split_once(":") {
                    Some((target_users, message)) => {
                        for user in target_users.split(",") {
                            let user = user.trim();
                            let guard = app.lock().await;
                            if let Some(channel) = guard.get_user_channel(user) {
                                channel.send(format!("from {username}:{message}")).await?;
                            }
                        }
                    }
                    _ => {}
                };
            }

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
