mod event_tap;
mod state;
mod ui;

use core_foundation::runloop::CFRunLoop;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use state::AppState;
use std::{
    io,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_for_tap = Arc::clone(&running);
    let running_for_ui = Arc::clone(&running);

    let state = Arc::new(RwLock::new(AppState::new()));
    let state_for_tap = Arc::clone(&state);

    let ui_thread = thread::spawn(move || {
        enable_raw_mode().unwrap();
        execute!(io::stdout(), EnterAlternateScreen).unwrap();

        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).unwrap();

        loop {
            terminal
                .draw(|f| {
                    let s = state.read().unwrap();
                    ui::draw(f, &s);
                })
                .unwrap();

            if event::poll(std::time::Duration::from_millis(16)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Char('q') => {
                            running_for_ui.store(false, Ordering::SeqCst);
                            CFRunLoop::get_main().stop();
                            break;
                        }
                        KeyCode::Char('p') => {
                            let mut s = state.write().unwrap();
                            s.paused = !s.paused;
                        }
                        KeyCode::Char('c') => {
                            let mut s = state.write().unwrap();
                            s.clear();
                        }
                        _ => {}
                    }
                }
            }
        }

        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    });

    event_tap::start_event_tap(state_for_tap, running_for_tap);

    ui_thread.join().unwrap();
}
