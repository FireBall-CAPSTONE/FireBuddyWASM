use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32) {
    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new( AppState {
    });
}

pub struct AppState {

}

impl AppState {
    pub fn new() -> Self {
        Self {

        }
    }
}