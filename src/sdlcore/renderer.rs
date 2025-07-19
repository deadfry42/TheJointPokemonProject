use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct GameRenderer {
    pub canvas: Canvas<Window>,
}

impl GameRenderer {
    pub fn new(canvas: Canvas<Window>) -> GameRenderer {
        GameRenderer { canvas: canvas }
    }

    pub fn draw(
        &mut self,
        texture: &Texture,
        spritemap_location: Option<Rect>,
        window_location: Option<Rect>,
    ) {
        self.canvas
            .copy(texture, spritemap_location, window_location)
            .unwrap();
    }

    pub fn finish_frame(&mut self) {
        self.canvas.present();
    }
}
