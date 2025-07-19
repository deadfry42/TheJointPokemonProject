use crate::GAME_CAMERA;

pub struct Camera {
    pub x: i64,
    pub y: i64,
}

impl Camera {
    pub fn new() -> Camera {
        Camera { x: 0, y: 0 }
    }

    pub fn set_x(x: i64) {
        GAME_CAMERA.lock().unwrap().x = x;
    }

    pub fn set_y(y: i64) {
        GAME_CAMERA.lock().unwrap().y = y;
    }

    pub fn get_x() -> i64 {
        GAME_CAMERA.lock().unwrap().x
    }

    pub fn get_y() -> i64 {
        GAME_CAMERA.lock().unwrap().y
    }
}
