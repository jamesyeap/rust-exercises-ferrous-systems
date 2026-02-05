use std::{io, net::{TcpListener, TcpStream}, thread};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), io::Error> {
    let log: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![]));

    let listener = TcpListener::bind("127.0.0.1:8080");
    for stream in listener?.incoming() {
        let log = log.clone(); // create a new reference to the log vector - increments the reference count
        thread::spawn(move || handle_client(stream?, log));
    }

    Ok(())
}

fn handle_client(stream: TcpStream, log: Arc<Mutex<Vec<usize>>>) -> Result<(), io::Error> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream.try_clone()?;

    let mut buffer = String::new();
    loop {
        buffer.clear();
        reader.read_line(&mut buffer);

        // blocks the current thread, until an exclusive lock can be acquired for the log vector
        if let Ok(mut log) = log.lock() {
           log.push(buffer.len());
            println!("{:?}", log);
        }
        writer.write_all(buffer.as_bytes())?;
    }
}
