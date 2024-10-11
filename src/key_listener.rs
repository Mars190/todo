use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use crate::app_state::{AppState, Mode};
use crate::service::Service;

pub struct KeyListener;

impl KeyListener {
    pub fn listen(&self, state: &mut AppState, service: &Service) {
        loop {
            let event = read().unwrap();
            if let Event::Key(key) = event {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match state.get_mode() {
                    Mode::Main => self.prototype_handle_keys(key, state, service),
                    _ => {}
                }
            }
        }
    }

    fn prototype_handle_keys(&self, key: KeyEvent, state: &mut AppState, service: &Service) {
        match key.code {
            KeyCode::Char('q') => Service::quit(),
            _ => {
                service.prototype(state, &key);
            }
        }
    }
}