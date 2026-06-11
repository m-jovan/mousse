use crate::state::AppState;
use core_foundation::runloop::CFRunLoop;
use core_graphics::event::{
    CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType,
    CallbackResult,
};
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};

pub fn start_event_tap(state: Arc<RwLock<AppState>>, running: Arc<AtomicBool>) {
    let tap = CGEventTap::new(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::ListenOnly,
        vec![
            CGEventType::MouseMoved,
            CGEventType::LeftMouseDown,
            CGEventType::LeftMouseUp,
            CGEventType::LeftMouseDragged,
            CGEventType::RightMouseDown,
            CGEventType::RightMouseUp,
            CGEventType::RightMouseDragged,
            CGEventType::OtherMouseDown,
            CGEventType::OtherMouseUp,
            CGEventType::OtherMouseDragged,
            CGEventType::ScrollWheel,
        ],
        move |_, event_type, event| {
            if !running.load(Ordering::SeqCst) {
                CFRunLoop::get_current().stop();
                return CallbackResult::Keep;
            }

            let mut s = state.write().unwrap();
            match event_type {
                CGEventType::MouseMoved
                | CGEventType::LeftMouseDragged
                | CGEventType::RightMouseDragged
                | CGEventType::OtherMouseDragged => {
                    let point = event.location();
                    s.mouse_x = point.x as i32;
                    s.mouse_y = point.y as i32;
                }
                CGEventType::LeftMouseDown => {
                    s.btn_left = true;
                    s.log("LEFT DOWN".to_string());
                }
                CGEventType::LeftMouseUp => {
                    s.btn_left = false;
                    s.log("LEFT UP".to_string());
                }
                CGEventType::RightMouseDown => {
                    s.btn_right = true;
                    s.log("RIGHT DOWN".to_string());
                }
                CGEventType::RightMouseUp => {
                    s.btn_right = false;
                    s.log("RIGHT UP".to_string());
                }
                CGEventType::OtherMouseDown => {
                    let button = event.get_integer_value_field(
                        core_graphics::event::EventField::MOUSE_EVENT_BUTTON_NUMBER,
                    );
                    match button {
                        2 => {
                            s.btn_middle = true;
                            s.log("MIDDLE DOWN".to_string());
                        }
                        3 => {
                            s.btn_side_back = true;
                            s.log("BACK DOWN".to_string());
                        }
                        4 => {
                            s.btn_side_forward = true;
                            s.log("FORWARD DOWN".to_string());
                        }
                        _ => {}
                    }
                }
                CGEventType::OtherMouseUp => {
                    let button = event.get_integer_value_field(
                        core_graphics::event::EventField::MOUSE_EVENT_BUTTON_NUMBER,
                    );
                    match button {
                        2 => {
                            s.btn_middle = false;
                            s.log("MIDDLE UP".to_string());
                        }
                        3 => {
                            s.btn_side_back = false;
                            s.log("BACK UP".to_string());
                        }
                        4 => {
                            s.btn_side_forward = false;
                            s.log("FORWARD UP".to_string());
                        }
                        _ => {}
                    }
                }
                CGEventType::ScrollWheel => {
                    let dx = event.get_integer_value_field(
                        core_graphics::event::EventField::SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS_2,
                    );
                    let dy = event.get_integer_value_field(
                        core_graphics::event::EventField::SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS_1,
                    );
                    let is_pixel = event.get_integer_value_field(
                        core_graphics::event::EventField::SCROLL_WHEEL_EVENT_IS_CONTINUOUS,
                    ) == 1;

                    s.scroll_dx = dx as f64;
                    s.scroll_dy = dy as f64;
                    s.scroll_is_pixel = is_pixel;

                    s.scroll_dy_history.push_back(dy as f64);
                    if s.scroll_dy_history.len() > 60 {
                        s.scroll_dy_history.pop_front();
                    }
                    s.scroll_dx_history.push_back(dx as f64);
                    if s.scroll_dx_history.len() > 60 {
                        s.scroll_dx_history.pop_front();
                    }

                    s.log(format!("SCROLL dy:{} dx:{}", dy, dx));
                }
                _ => {}
            }
            CallbackResult::Keep
        },
    )
    .unwrap();

    let current_loop = CFRunLoop::get_current();
    let loop_source = tap.mach_port().create_runloop_source(0).unwrap();

    current_loop.add_source(&loop_source, unsafe {
        core_foundation::runloop::kCFRunLoopCommonModes
    });

    tap.enable();
    CFRunLoop::run_current();
}
