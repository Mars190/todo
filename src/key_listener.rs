
use std::io::stdin;

use termion::event::Key;
use termion::input::TermRead;

use crate::app_state::{AppState, Mode};
use crate::service::Service;

pub struct KeyListener;

impl KeyListener {
    pub fn listen(&self, state: &mut AppState, service: &Service) {
        let stdin = stdin();

        for event in stdin.events() {
            let event = event.unwrap();

            if let Event::Key(key) = event {
                println!("[DEBUG] Key pressed");

                match state.get_mode() {
                    Mode::Main => self.prototype_handle_keys(key, state, service),
                    _ => {}
                }
            }
        }
    }

    fn prototype_handle_keys(&self, key: Key, state: &mut AppState, service: &Service) {
        match key {
            Key::Char('q') => Service::quit(),
            _ => {
                service.prototype(state, &key);
            }
        }
    }
}