use std::fmt;
use std::net::SocketAddr;
use std::collections::HashMap;

extern crate mio;
use mio::*;
use mio::net::{TcpListener};

pub struct Server {
    host_name: String,
    port_number: u32
}

impl Server {
    pub fn new(host_name: String, port_number: u32) -> Self {
        Server {host_name, port_number}
    }

    pub fn run(&self) {
        println!("The server is running");
        let mut address: String = self.host_name.clone();
        address.push_str(":");
        address.push_str(&self.port_number.to_string());
        let mut socket_address: SocketAddr = address.parse().unwrap();
        // Bind listening socket
        let server = TcpListener::bind(&socket_address).unwrap();

        // Create event listner using tokens
        let SERVER_TOKEN: Token = Token(0);
        let poll = Poll::new().unwrap();
        poll.register(&server, SERVER_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();
        let mut events = Events::with_capacity(1024);

    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.host_name, self.port_number)
    }
}