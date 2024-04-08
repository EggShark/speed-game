use bottomless_pit::render::RenderInformation;
use bottomless_pit::{engine_handle::Engine, material::Material};
use bottomless_pit::input::MouseKey;
use bottomless_pit::{material, vec2};
use bottomless_pit::vectors::Vec2;

use crate::level::{Level, Platform};

pub trait Tool {
    fn on_click(&mut self, mouse_pos: Vec2<f32>, level: &mut Level);
    fn on_mouse_release(&mut self, mouse_pos: Vec2<f32>, level: &mut Level);
    fn update(&mut self, engine: &mut Engine, level: &mut Level);
    fn draw(&self, material: &mut Material, renderer: &mut RenderInformation);
}

#[derive(Debug)]
pub struct PlatformTool {
    selection: Vec<usize>,
    mouse_pressed_pos: Vec2<f32>,
    mouse_down: bool,
    preview_platform: Option<Platform>,
}

impl PlatformTool {
    pub fn new() -> Self {
        Self {
            selection: vec![],
            mouse_pressed_pos: vec2!(0.0),
            mouse_down: false,
            preview_platform: None,
        }
    }
}

impl Tool for PlatformTool {
    fn on_click(&mut self, mouse_pos: Vec2<f32>, level: &mut Level,) {
        self.mouse_pressed_pos = mouse_pos;
    }

    fn on_mouse_release(&mut self, _: Vec2<f32>, level: &mut Level) {
        let p = self.preview_platform.take().unwrap();

        level.add_platform(p);
    }

    fn update(&mut self, engine: &mut Engine, level: &mut Level) {
        self.mouse_down = engine.is_mouse_key_down(MouseKey::Left);

        if self.mouse_down {
            self.preview_platform = Some(Platform::from_corners(self.mouse_pressed_pos, engine.get_mouse_position()));
        }
    }

    fn draw(&self, material: &mut Material, renderer: &mut RenderInformation) {
        if let Some(p) = &self.preview_platform {
            p.draw(material, renderer);
        }
    }
}

pub struct Selector {
    selection: Vec<usize>,
    mouse_pressed_pos: Vec2<f32>,
    mouse_down: bool,
}

impl Selector {
    pub fn new() -> Self {
        Self {
            selection: Vec::new(),
            mouse_pressed_pos: vec2!(0.0),
            mouse_down: false,
        }
    }
}

impl Tool for Selector {
    fn update(&mut self, engine: &mut Engine, level: &mut Level) {

    }

    fn on_click(&mut self, mouse_pos: Vec2<f32>, level: &mut Level) {
        
    }

    fn on_mouse_release(&mut self, mouse_pos: Vec2<f32>, level: &mut Level) {
        
    }

    fn draw(&self, material: &mut Material, renderer: &mut RenderInformation) {
        
    }
}