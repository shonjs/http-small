use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs::File;
use std::path::Path;
use std::env;

extern crate getopts;
use getopts::Options;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("p", "port", "listening port", "PORT");

    let matches = match opts.parse(&args[1..]) {
        Ok(matches) => {matches},
        Err(err) => { panic!("Error: {}", err) }
    };

    if !matches.opt_present("p") {
        println!("Port needs to be specified");
        return
    }

    let port: u32 = matches.opt_str("p").unwrap().parse::<u32>().unwrap();
    let bind_address: String = String::from("0.0.0.0:").push_str(&port.to_string());
    let listener = TcpListener::bind(bind_address).unwrap();
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
    let file = File::open(Path::new("./assets/test.txt"));
    // let mut buffer = [0;4096];

    // file.read(&mut buffer);

    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer)?;
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    let mut buffer = String::new();
    match file {
        Ok(mut reader) => {
            reader.read_to_string(&mut buffer).unwrap();
            // let response = buffer.as_bytes();
            match stream.write(response) {
                Ok(_) => println!("Response sent"),
                Err(error) => println!("failed sending response")
            }
        }
        Err(error) => println!("Error in opening file {}", error)
    }
}
