use bottomless_pit::colour::Colour;
use bottomless_pit::input::MouseKey;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::Game;
use bottomless_pit::vec2;
use bottomless_pit::vectors::Vec2;
use bottomless_pit::engine_handle::{Engine, EngineBuilder};
use bottomless_pit::render::RenderInformation;
use level_editor::level::Platform;

fn main() {
    let mut engine = EngineBuilder::new()
        .build()
        .unwrap();

    let material = MaterialBuilder::new()
        .build(&mut engine);

    let editor = Editor::new(material);

    engine.run(editor);
}


struct Editor {
    mouse_pressed_pos: Vec2<f32>,
    mouse_down: bool,
    current_mouse_pos: Vec2<f32>,
    material: Material,
    platforms: Vec<Platform>,
    preview_platform: Option<Platform>,
}

impl Editor {
    pub fn new(mat: Material) -> Self {
        Self {
            mouse_pressed_pos: vec2!(0.0),
            mouse_down: false,
            current_mouse_pos: vec2!(0.0),
            material: mat,
            platforms: Vec::new(),
            preview_platform: None,
        }
    }
}

impl Game for Editor {
    fn update(&mut self, engine_handle: &mut Engine) {
        if engine_handle.is_mouse_key_pressed(MouseKey::Left) {
            self.mouse_pressed_pos = engine_handle.get_mouse_position();
            self.preview_platform = Some(Platform::from_corners(self.mouse_pressed_pos, self.mouse_pressed_pos));
        }

        if engine_handle.is_mouse_key_down(MouseKey::Left) {
            self.mouse_down = true;
            self.preview_platform = Some(Platform::from_corners(self.mouse_pressed_pos, engine_handle.get_mouse_position()));
        }

        if engine_handle.is_mouse_key_released(MouseKey::Left) {
            self.mouse_down = false;
            self.platforms.push(self.preview_platform.take().unwrap());
        }
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>)
        where
            'others: 'pass {
        for p in &self.platforms {
            p.draw(&mut self.material, &renderer);
        }

        match &self.preview_platform {
            Some(p) => p.draw(&mut self.material, &renderer),
            None => {},
        }

        self.material.draw(&mut renderer);
    }
}