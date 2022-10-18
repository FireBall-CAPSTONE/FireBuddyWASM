use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::math::mat4::Matrix4;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new(0.0, 800.0, 600.0)));
}

lazy_static! {
    static ref MAT_STACK: Mutex<Vec<Matrix4>> = Mutex::new(Vec::new());
}

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32) {
    let mut data = APP_STATE.lock().unwrap();
    
    *data = Arc::new( AppState::new(time, canvas_height, canvas_width));
}

/// Takes in a matrix and multiplies it onto the mat stack
pub fn multiply_to_mat_stack(mat: Matrix4) {
    let mut stack = MAT_STACK.lock().unwrap();
    let top = *stack.last().unwrap_or(&Matrix4::identity());

    stack.push(
        mat * top
    )
}

pub fn pop_from_mat_stack() {
    // TODO: Pop from the mat stack
    MAT_STACK.lock().unwrap().pop();

}

pub fn peek_mat_stack() -> Matrix4 {
    *MAT_STACK.lock().unwrap().last().unwrap_or(&Matrix4::identity())
}

pub struct AppState {
    time: f32,
    canvas_height: f32,
    canvas_width: f32,
    pub(crate) mat_stack: Vec<Matrix4>
}

impl AppState {
    pub fn new(time: f32, canvas_height: f32, canvas_width: f32) -> Self {
        Self {
            time: time,
            canvas_height: canvas_height,
            canvas_width: canvas_width,
            mat_stack: Vec::new()
        }
    }

    pub(crate) fn push_to_mat_sack(self, m: Matrix4) -> Self {
        Self::new(self.time, self.canvas_height, self.canvas_width)
    }
}