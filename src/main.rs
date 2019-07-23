use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_read(&stream));
            }
            Err(err) => {
                println!("Error in connection {}", err);
            }
        }
    }
}

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buf);
            println!("{}", request);
        }
        Err(err) => {
            println!("Reading stream failed {}", err);
        }
    }
}
