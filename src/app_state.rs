use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::graphics::camera::Camera;
use crate::math::mat4::Matrix4;
use crate::math::vec2::Vector2;
use crate::math::vec3::Vector3;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new(800.0, 600.0)));
}

lazy_static! {
    static ref MAT_STACK: Mutex<Vec<Matrix4>> = Mutex::new(Vec::new());
}

lazy_static! {
    static ref MOUSE_STATE: Mutex<Arc<MouseState>> = Mutex::new(Arc::new(MouseState::new()));
}

lazy_static! {
    static ref CAMERA: Mutex<Arc<Camera>> = Mutex::new(Arc::new(Camera::new()));
}

pub fn update_dynamic_data(canvas_height: f32, canvas_width: f32) {
    let mut data = APP_STATE.lock().unwrap();
    
    *data = Arc::new( AppState::new(canvas_height, canvas_width));
}

pub fn get_canvas_height() -> f32 {
    APP_STATE.lock().unwrap().canvas_height
}

pub fn get_canvas_width() -> f32 {
    APP_STATE.lock().unwrap().canvas_width
}

pub fn set_mouse_pos(x: f32, y: f32) {
    let mut mouse_state = MOUSE_STATE.lock().unwrap();

    *mouse_state = Arc::new( MouseState {
        x,
        y,
        delta_x: mouse_state.delta_x,
        delta_y: mouse_state.delta_y,
        down: mouse_state.down,
        drag: mouse_state.drag
    } );
}

pub fn get_mouse_pos() -> Vector2 {
    let mouse_state = MOUSE_STATE.lock().unwrap();

    Vector2::new(
        mouse_state.x,
        mouse_state.y
    )
}

pub fn update_mouse_delta(delta_x: f32, delta_y: f32) {
    // This should exclusively be called in update
    let mut mouse_state = MOUSE_STATE.lock().unwrap();

    let delta = mouse_state.down as i32 as f32;

    *mouse_state = Arc::new( MouseState {
        x: mouse_state.x,
        y: mouse_state.y,
        delta_x: delta_x * delta,
        delta_y: delta_y * delta,
        down: mouse_state.down,
        drag: mouse_state.drag
    });
}

pub fn get_mouse_delta() -> Vector2 {
    
    let mouse_state = MOUSE_STATE.lock().unwrap();

    Vector2::new(
        mouse_state.delta_x,
        mouse_state.delta_y
    )
}

pub fn set_mouse_down(down: bool) {
    let mut mouse_state = MOUSE_STATE.lock().unwrap();

    *mouse_state = Arc::new( MouseState {
        x: mouse_state.x,
        y: mouse_state.y,
        delta_x: mouse_state.delta_x,
        delta_y: mouse_state.delta_y,
        down,
        drag: mouse_state.drag
    });
}

pub fn set_mouse_drag(drag: bool) {
    let mut mouse_state = MOUSE_STATE.lock().unwrap();

    *mouse_state = Arc::new( MouseState {
        drag: drag&&mouse_state.down,
        ..*mouse_state.as_ref() // much cleaner
    })
}

pub fn get_mouse_drag() -> bool {
    let mouse_state = MOUSE_STATE.lock().unwrap();

    mouse_state.drag
}

pub fn get_projection_matrix() -> Matrix4 {
    let cam = CAMERA.lock().unwrap();

    cam.proj_matrix
}

pub fn get_view_matrix() -> Matrix4 {
    let cam = CAMERA.lock().unwrap();

    cam.view_matrix
}

pub fn update_projection_matrix() {
    // configure the new projection matrix
    let mut cam = CAMERA.lock().unwrap();
    let aspect_ratio = get_canvas_width() / get_canvas_height();
    let proj_mat = Matrix4::perspective(
        cam.fov, aspect_ratio, cam.near, cam.far);

    *cam = Arc::new( Camera {
        proj_matrix: proj_mat,
        ..*cam.as_ref()
    } );

}

pub fn update_view_matrix() {
    // configure the new view matrix
    let mut cam = CAMERA.lock().unwrap();
    let view_mat = Matrix4::view(
        cam.position,
        Vector3::up(),
        Vector3::forward()
    );

    *cam = Arc::new( Camera {
        view_matrix: view_mat,
        ..*cam.as_ref()
    });
}

pub fn set_camera_position(pos: Vector3) {
    let mut cam = CAMERA.lock().unwrap();
    *cam = Arc::new( Camera {
        position: pos,
        ..*cam.as_ref()
    });
}

pub fn move_camera(delta: Vector3) {
    let mut cam = CAMERA.lock().unwrap();
    *cam = Arc::new( Camera {
        position: cam.position + delta,
        ..*cam.as_ref()
    });
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
    MAT_STACK.lock().unwrap().pop();

}

pub fn peek_mat_stack() -> Matrix4 {
    *MAT_STACK.lock().unwrap().last().unwrap_or(&Matrix4::identity())
}

pub struct AppState {
    canvas_height: f32,
    canvas_width: f32,
}

impl AppState {
    pub fn new(canvas_height: f32, canvas_width: f32) -> Self {
        Self {
            canvas_height: canvas_height,
            canvas_width: canvas_width,
        }
    }
}

pub struct MouseState {
    x: f32,
    y: f32,
    delta_x: f32,
    delta_y: f32,
    down: bool,
    drag: bool
}

impl MouseState {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, delta_x: 0.0, delta_y: 0.0, down: false, drag: false }
    }
}
