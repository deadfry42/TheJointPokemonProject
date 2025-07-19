use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct GameRenderer {
    pub canvas: Canvas<Window>,
}

impl GameRenderer {
    pub fn new(canvas: Canvas<Window>) -> GameRenderer {
        GameRenderer { canvas: canvas }
    }
}
