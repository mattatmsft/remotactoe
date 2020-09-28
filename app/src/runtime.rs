use console::{style, Term};
use rand::prelude::*;

mod configuration;
// mod network;

pub struct Runtime {
    canvas_width: u16,
    canvas_height: u16,
    config: configuration::Configuration,
    game_complete: bool,
    game_won: bool,
    display: Term,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            canvas_height: 10,
            canvas_width: 10,
            config: configuration::Configuration {
                // todo
            },
            game_complete: false,
            game_won: false,
            display: Term::stdout(),
        }
    }
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // todo
        self.display.clear_screen()?;
        let (heigth, width) = self.display.size();

        Ok(())
    }

    fn draw(&self) {
        // todo
    }
}