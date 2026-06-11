use chrono::Local;
use std::collections::VecDeque;

pub struct AppState {
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub btn_left: bool,
    pub btn_right: bool,
    pub btn_middle: bool,
    pub btn_side_back: bool,
    pub btn_side_forward: bool,
    pub scroll_dx: f64,
    pub scroll_dy: f64,
    pub scroll_is_pixel: bool,
    pub scroll_dy_history: VecDeque<f64>,
    pub scroll_dx_history: VecDeque<f64>,
    pub event_log: VecDeque<String>,
    pub paused: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mouse_x: 0,
            mouse_y: 0,
            btn_left: false,
            btn_right: false,
            btn_middle: false,
            btn_side_back: false,
            btn_side_forward: false,
            scroll_dx: 0.0,
            scroll_dy: 0.0,
            scroll_is_pixel: false,
            scroll_dy_history: VecDeque::with_capacity(60),
            scroll_dx_history: VecDeque::with_capacity(60),
            event_log: VecDeque::with_capacity(100),
            paused: false,
        }
    }

    pub fn log(&mut self, msg: String) {
        if self.paused {
            return;
        }
        let timestamp = Local::now().format("%H:%M:%S%.3f").to_string();
        self.event_log.push_back(format!("[{}] {}", timestamp, msg));
        if self.event_log.len() > 100 {
            self.event_log.pop_front();
        }
    }

    pub fn clear(&mut self) {
        self.scroll_dy_history.clear();
        self.scroll_dx_history.clear();
        self.event_log.clear();
    }
}
