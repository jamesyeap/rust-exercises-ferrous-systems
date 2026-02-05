use simple_db::{Command, parse};
use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;
use std::{
    collections::VecDeque,
    net::{TcpListener, TcpStream},
    thread,
};

fn main() -> Result<(), anyhow::Error> {
    let app = Application::new();
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    thread::scope(|s| {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    s.spawn(|| {
                        if let Err(error) = handle_client(stream, &app) {
                            println!("error encountered in handle_client function: {error}");
                        }
                    });
                }
                Err(_) => {}
            }
        }
    });

    Ok(())
}

fn handle_client(stream: TcpStream, app: &Application) -> Result<(), anyhow::Error> {
    let reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream.try_clone()?;

    for line in reader.lines() {
        let line = line?;

        match parse(&format!("{}\n", &line)) {
            Ok(command) => {
                if let Some(response) = app.execute(command) {
                    writer.write(format!("{response}\n").as_bytes());
                }
            }
            Err(error) => {
                writer.write(format!("{error}\n").as_bytes());
            }
        }
    }

    Ok(())
}

struct Application {
    messages: Mutex<VecDeque<String>>,
}

impl Application {
    fn new() -> Self {
        Self {
            messages: Mutex::new(VecDeque::new()),
        }
    }

    fn execute(&self, command: Command) -> Option<String> {
        match command {
            Command::Publish(message) => {
                if let Ok(mut messages) = self.messages.lock() {
                    messages.push_front(message);
                    Some("Published 1 message\n".to_string())
                } else {
                    None
                }
            }

            Command::Retrieve => {
                if let Ok(mut messages) = self.messages.lock() {
                    match messages.pop_back() {
                        Some(message) => Some(format!("{message}\n").to_string()),
                        None => Some("No messages!\n".to_string()),
                    }
                } else {
                    None
                }
            }
        }
    }
}
