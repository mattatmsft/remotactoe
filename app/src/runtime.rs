use console::{style, Term};
use rand::prelude::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

mod configuration;
// mod network;

pub struct Runtime {
    canvas_width: u16,
    canvas_height: u16,
    config: configuration::Configuration,
    game_complete: bool,
    game_won: bool,
    display: Term,
    game_state: Vec<char>,
}

impl Runtime {
    // pub fn new() -> Runtime {
    //     let mut runtime = Runtime {
    //         canvas_height: 10,
    //         canvas_width: 10,
    //         config: configuration::Configuration { is_host: false },
    //         game_complete: false,
    //         game_won: false,
    //         display: Term::stdout(),
    //         game_state: vec![],
    //     };

    //     for n in 0..8 {
    //         runtime.game_state.push(' ');
    //     }

    //     runtime
    // }

    pub fn new(terminal: Term, is_host: bool) -> Runtime {
        let mut runtime = Runtime {
            canvas_height: 10,
            canvas_width: 10,
            config: configuration::Configuration::new(),
            game_complete: false,
            game_won: false,
            display: terminal,
            game_state: vec![],
        };

        for n in 0..8 {
            runtime.game_state.push(' ');
        }

        runtime.config.is_host = is_host;

        runtime
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // todo
        self.display.clear_screen()?;
        let (heigth, width) = self.display.size();

        self.display.write_str(&"Loading...")?;
        sleep(Duration::from_secs(1));

        // initialize game session
        if self.config.is_host {
            // to do: initialize connection
            let listener = TcpListener::bind(format!("127.0.0.1:{}", self.config.host_port))?;
            let responese_message = String::from("Hello from the host.");

            for stream in listener.incoming() {
                let mut stream = stream.unwrap();

                loop {
                    // let mut receive_response = String::new();
                    let mut host_buffer = [0; 1024];
                    stream.read(&mut host_buffer)?;
                    
                    println!("Server received: {}", str::from_utf8(&host_buffer).unwrap());
                    stream.write(&mut responese_message.as_bytes())?;
                    stream.flush()?;
                    sleep(Duration::from_secs(3));
                }
            }
        } else {
            // we are now a client, make is so.
            // listener = TcpListener::bind(format!("127.0.0.1{}", self.config.host_port))?;
            let mut stream = TcpStream::connect(format!("127.0.0.1:{}", self.config.host_port))
                .expect("Couldn't connect to the server!");

            let sender = String::from("Hello there Mr. Wilson.");
            loop {
                stream.write(sender.as_bytes())?;
                stream.flush()?;
                let mut client_buffer = [0; 1024];
                stream.read(&mut client_buffer)?;
                println!("Response: {}", str::from_utf8(&client_buffer).unwrap());

                sleep(Duration::from_secs(3));
            }
        }

        while !self.game_complete {
            // game loop

            self.game_complete = true;
        }

        self.draw();

        Ok(())
    }

    fn draw(&self) {
        let mut index = 0;
        self.display.clear_screen().unwrap();
        self.display.move_cursor_to(0, 0).unwrap();

        for n in 0..5 {
            let mut display_line = String::new();

            if n % 2 == 0 {
                for p in 0..3 {
                    match p {
                        0 => display_line.push(self.game_state[index]),
                        1 => {
                            display_line.push('|');
                            display_line.push(self.game_state[index]);
                        }
                        2 => {
                            display_line.push('|');
                            display_line.push(self.game_state[index]);
                        }
                        _ => println!("nothing here!"),
                    }
                }
            } else {
                display_line.push('-');
                display_line.push('-');
                display_line.push('-');
                display_line.push('-');
                display_line.push('-');
            }
            self.display.write_str(&display_line).unwrap();
            self.display.move_cursor_to(0, n + 1).unwrap();
            index += 1;
        }

        // todo: remove
        sleep(Duration::from_secs(5));
    }

    fn initialize_client(&self) {
        // this is where i collect the ip information.
    }
}
