use std::{io, net::{TcpListener, TcpStream}, thread};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080");

    for stream in listener?.incoming() {
        thread::spawn(|| handle_client(stream?));
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream.try_clone()?;
    let mut buffer = String::new();

    loop {
        buffer.clear();
        reader.read_line(&mut buffer);

        writer.write_all(buffer.as_bytes())?;
    }
}
