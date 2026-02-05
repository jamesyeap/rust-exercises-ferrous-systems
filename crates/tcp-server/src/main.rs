use std::io::{BufRead, BufReader, Write};
use std::{
    io,
    net::{TcpListener, TcpStream},
    thread,
};
use std::sync::Mutex;

fn main() -> Result<(), io::Error> {
    let log: Mutex<Vec<usize>> = Mutex::new(vec![]);
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    thread::scope(|s| {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    s.spawn(|| {
                        if let Err(error) = handle_client(stream, &log) {
                            println!("error encountered in handle_client function: {error}");
                        }
                    });
                }
                Err(_) => {}
            }
        }
    });

    println!("log vec: {:?}", log);

    Ok(())
}

fn handle_client(stream: TcpStream, log: &Mutex<Vec<usize>>) -> Result<(), io::Error> {
    let reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream.try_clone()?;

    for line in reader.lines() {
        let line = line?;

        if let Ok(mut log) = log.lock() {
            log.push(line.len());
            println!("{:?}", log);
        }
        writer.write_all(line.as_bytes());
        writer.write_all("\n".as_bytes());
    }

    Ok(())
}
