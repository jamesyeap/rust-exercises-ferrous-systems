use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter wrapped in Arc (for sharing) and Mutex (for exclusive access)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Critical section begins here
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented to {}", i, *num);
            // Critical section ends when `num` (the guard) is dropped
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}

/*

use std::{io, net::{TcpListener, TcpStream}, thread};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        reader.read_line(&mut buffer);

        let mut writer = reader.get_mut();
        writer.write_all(buffer.as_bytes())?;
        writer.flush();
    }
}

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080");

    let mut handles = Vec::new();
    for stream in listener?.incoming() {
        let handle = thread::spawn(|| handle_client(stream?));
        handles.push(handle);
    }

    for handle in handles.into_iter() {
        match handle.join() {
            Ok(_) => println!("Thread has finished!"),
            Err(error) => eprintln!("Error encountered in thread - error: {:?}", error),
        }
    }

    Ok(())
}

*/