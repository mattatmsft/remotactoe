use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Server {
    pub is_hosting: bool,
}

impl Server {
    pub fn new(&self) -> Server{
        Server {}
    }

    pub fn start(&self) {
        // todo
    }
}

pub struct Client {
    pub is_connected: bool,
}

impl Client {
    pub fm new(&self) -> Client {
        Client {}
    }

    pub fn connect (&self, address: String) {
        // todo
    }

    pub fn send(&self, content: String) {
        // todo
    }
}