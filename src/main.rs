mod app_state;
mod todo;
mod key_listener;
mod service;

use app_state::AppState;
use key_listener::KeyListener;
use service::Service;

fn main() {
    println!("Starting app...");

    let key_listener = KeyListener {};
    let mut state = AppState::new();
    let service = Service::new();
    
    key_listener.listen(&mut state, &service);
}