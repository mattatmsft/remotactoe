use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub trait Connection {
    fn send(&self, message: String) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Server {
    pub is_hosting: bool,
    listener: Option<TcpListener>,
    stream: Option<TcpStream>,
}

impl Server {
    pub fn open_connection(address: String) -> Result<Server, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(address)?;
        
        let connection = Server {
            is_hosting: false,
            listener: Some(listener),
            stream: None,
        };

        Ok(connection)
    }
}

pub struct Client {
    pub is_connected: bool,
}

impl Client {
    pub fn connect (&self, address: String) {
        // todo
    }

    pub fn send(&self, content: String) {
        // todo
    }
}