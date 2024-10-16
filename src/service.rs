use std::{io::stdout, process::ExitCode};

use crossterm::{event::{KeyEvent, KeyModifiers}, terminal, ExecutableCommand};

use crate::app_state::AppState;

pub struct Service;

impl Service {
    pub fn new() -> Service {
        Service {}
    }
    
    pub fn prototype(&self, state: &mut AppState, key: &KeyEvent) {
        println!("{}", state);
        println!("{}", Service::parse_key_event(key));
        println!("");
    }

    pub fn quit() {
        std::process::exit(0);
    }

    fn parse_key_event(key: &KeyEvent) -> String {
        let mut output = String::from("Key pressed: ");

        if key.modifiers != KeyModifiers::NONE {
            output.push_str(&format!("{}+", key.modifiers));
        }
        output.push_str(&format!("{}\r", key.code));

        output
    }
}