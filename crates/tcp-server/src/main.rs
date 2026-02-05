use std::io::{BufRead, BufReader, Write};
use std::{
    io,
    net::{TcpListener, TcpStream},
    thread,
};
use std::sync::mpsc::{channel, Sender};

fn main() -> Result<(), io::Error> {
    let mut log: Vec<usize> = vec![];
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let (sender, receiver) = channel::<String>();

    thread::spawn(move || {
        loop {
            if let Ok(message) = receiver.recv() {
                log.push(message.len());
                println!("vector is now: {:?}", log);
            }
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let sender = sender.clone();
                thread::spawn(|| {
                    if let Err(error) = handle_client(stream, sender) {
                        println!("error encountered in handle_client function: {error}");
                    }
                });
            }
            Err(_) => {}
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream, sender: Sender<String>) -> Result<(), io::Error> {
    let reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream.try_clone()?;

    for line in reader.lines() {
        let line = line?;

        sender.send(line.clone());
        writer.write_all(line.as_bytes());
        writer.write_all("\n".as_bytes());
    }

    Ok(())
}
