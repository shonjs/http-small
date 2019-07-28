use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs::File;
use std::path::Path;
use std::env;

extern crate getopts;
use getopts::Options;

mod server;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("p", "port", "listening port", "PORT");
    opts.optopt("n", "hostname", "name of host machine ", "HOST");

    let matches = match opts.parse(&args[1..]) {
        Ok(matches) => {matches},
        Err(err) => { panic!("Error: {}", err) }
    };

    if !matches.opt_present("p") {
        println!("Port needs to be specified");
        return
    }

    if !matches.opt_present("n") {
        println!("Host not specified. Defaults to localhost");
    }

    let port_number: u32 = matches.opt_str("p").unwrap().parse::<u32>().unwrap();
    let host_name = matches.opt_str("n");

    let mut bind_address: String = match host_name {
        Some(host) => host.parse::<String>().unwrap(),
        None => String::from("0.0.0.0")
    };
    bind_address.push(':');
    bind_address.push_str(&port_number.to_string());
    let listener = TcpListener::bind(bind_address).unwrap();
    println!("Listening on port {}", &port_number.to_string());

    let server: server::Server = server::Server::new(String::from("0.0.0.0"), port_number);
    println!("server {}", server);
    server.run();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_request(stream));
            }
            Err(err) => {
                println!("Error in connection {}", err);
            }
        }
    };
}

fn handle_request(stream: TcpStream){
    handle_read(&stream);
    read_response(stream);
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
                Err(_error) => println!("failed sending response")
            }
        }
        Err(error) => println!("Error in opening file {}", error)
    }
}
