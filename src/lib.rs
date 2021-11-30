use std::cell::RefCell;
use std::rc::Rc;

use chrono::Timelike;
use eframe::wasm_bindgen::{self, prelude::*};

use crate::fractal_clock::{FractalClock, FractalClockSettings};
use crate::wrap_app::WrapApp;

mod fractal_clock;
mod wrap_app;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ClockApp {
    clock: Rc<RefCell<FractalClock>>,
}

impl Default for ClockApp {
    fn default() -> Self {
        ClockApp {
            clock: Rc::new(RefCell::new(FractalClock::default())),
        }
    }
}

#[wasm_bindgen]
impl ClockApp {
    pub fn start(&self, canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
        let app = WrapApp { clock: self.clock.clone() };
        eframe::start_web(canvas_id, Box::new(app))
    }

    pub fn import_settings(&mut self, settings: &FractalClockSettings) {
        self.clock.borrow_mut().apply_clock_settings(settings);
    }
}

#[wasm_bindgen]
pub fn initialize_app() -> ClockApp {
    ClockApp::default()
}

/// Time of day as seconds since midnight
pub(crate) fn seconds_since_midnight() -> Option<f64> {
    let time = chrono::Local::now().time();
    let seconds_since_midnight =
        time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64);
    Some(seconds_since_midnight)
}