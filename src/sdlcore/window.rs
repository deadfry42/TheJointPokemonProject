extern crate sdl2;
extern crate sdl2_sys;

use sdl2::AudioSubsystem;
use sdl2::IntegerOrSdlError;
use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::render::Canvas;
use sdl2::render::CanvasBuilder;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowBuildError;
use sdl2::video::WindowContext;

use crate::sdlcore::renderer::GameRenderer;

pub struct GameWindow {
    pub sdl_context: Sdl,
    pub img_context: Sdl2ImageContext,
    pub video_subsystem: VideoSubsystem,
    pub audio_subsystem: AudioSubsystem,
    pub texture_creator: TextureCreator<WindowContext>,
    pub renderer: GameRenderer,
}

#[allow(dead_code)]
impl GameWindow {
    pub fn new() -> Result<GameWindow, String> {
        let sdl_context: Sdl = sdl2::init()?;
        let img_context: Sdl2ImageContext = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
        let video_subsystem: VideoSubsystem = sdl_context.video()?;
        let audio_subsystem: AudioSubsystem = sdl_context.audio()?;

        let window: Result<Window, WindowBuildError> = video_subsystem
            .window("TJPP", 1280, 720)
            .vulkan()
            .allow_highdpi()
            .position_centered()
            .build();

        if window.is_err() {
            return Err(String::from("Failed to build window!"));
        }

        let canvas_builder: Result<Canvas<Window>, IntegerOrSdlError> =
            CanvasBuilder::new(window.unwrap()).accelerated().build();

        if canvas_builder.is_err() {
            return Err(String::from("Failed to build canvas!"));
        }

        let canvas = canvas_builder.unwrap();
        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        let game_renderer = GameRenderer::new(canvas);

        Ok(GameWindow {
            sdl_context: sdl_context,
            img_context: img_context,
            video_subsystem: video_subsystem,
            audio_subsystem: audio_subsystem,
            texture_creator: texture_creator,
            renderer: game_renderer,
        })
    }

    pub fn get_window(&self) -> &Window {
        self.renderer.canvas.window()
    }

    pub fn get_window_mut(&mut self) -> &Window {
        self.renderer.canvas.window_mut()
    }
}
