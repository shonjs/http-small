use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs::File;
use std::path::Path;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_request(stream));
            }
            Err(err) => {
                println!("Error in connection {}", err);
            }
        }
    }
}

fn handle_request(stream: TcpStream){
    handle_read(&stream);
    read_response(stream)
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

fn read_response(mut stream: TcpStream) {
    let file = File::open(Path::new("../assets/image.jpeg"));
    // let mut buffer = [0;4096];

    // file.read(&mut buffer);

    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer)?;

    let mut buffer = String::new();
    match file {
        Ok(mut reader) => {
            reader.read_to_string(&mut buffer).unwrap();
            let response = buffer.as_bytes();
            match stream.write(response) {
                Ok(_) => println!("Response sent"),
                Err(error) => println!("failed sending response")
            }
        }
        Err(error) => println!("Error in opening file")
    }
}
