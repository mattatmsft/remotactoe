use console::Term;
use std::thread::sleep;
use std::time::Duration;

mod runtime;

fn main() {
    let terminal = Term::stdout();
    terminal.clear_screen().unwrap();
    terminal.write_str("Welcome to MWSoft's RemoTacToe, tic tac toe with a network twist.").unwrap();
    sleep(Duration::from_secs(3));

    let mut valid_input = false;
    let mut is_host = false;

    while !valid_input {
        terminal.clear_screen().unwrap();
        println!("Would you like to play a game?");
        println!("1) Host");
        println!("2) Connect");

        let mut response = String::new();
        std::io::stdin().read_line(&mut response).unwrap();

        match response.as_str().trim() {
            "1" => {
                // start server
                is_host = true;
                valid_input = true;
            },
            "2" => {
                // connect to host
                valid_input = true;
            },
            _ => {
                println!("Unexpected input, please try again.");
                sleep(Duration::from_secs(5));
            }
        }
    }

    let mut runtime = runtime::Runtime::new(terminal, is_host);
    runtime.start().unwrap();
}
