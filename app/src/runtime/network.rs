use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

// pub trait Connection {
//     fn send(&self, message: String) -> Result<(), Box<dyn std::error::Error>>;
// }

pub struct Connection {
    pub is_hosting: bool,
    listener: Option<TcpListener>,
    stream: Option<TcpStream>,
    receive_queue: Vec<String>,
    sender: Option<mpsc::Sender<String>>,
    listening_thread: Option<thread::JoinHandle<()>>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            is_hosting: false,
            listener: None,
            stream: None,
            receive_queue: Vec::new(),
            sender: None,
            listening_thread: None,
        }
    }

    pub fn host(&mut self, address: String) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(address).unwrap();
        let (sender, receiver) = mpsc::channel();

        let thread = thread::spawn(move || {
            let message = receiver.recv().unwrap();

            // match message {
            //     Message::NewJob(command) => {
            //         println!("Worker got a job; executing.");

            //         self.receive_queue.push(command.to_string());
            //     }
            //     Message::Terminate => {
            //         println!("Worker was told to terminate.");
            //         break;
            //     }
            // }
        });

        self.listening_thread = Some(thread);
        self.sender = Some(sender);
        
        for stream in listener.incoming() {
            let stream = stream?;
            self.stream = Some(stream);
            break;
        }

        Ok(())
    }

    pub fn connect (&self, address: String) {
        // todo
    }

    pub fn handle_connection<F>(&self, f: F) where F: Fn(), F: Send {
    }
}

// type Job = Box<dyn ToString + Send + 'static>;

// enum Message {
//     NewJob(Job),
//     Terminate,
// }
