use std::fmt;

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
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.host_name, self.port_number)
    }
}