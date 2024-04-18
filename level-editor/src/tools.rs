use bottomless_pit::colour::Colour;
use bottomless_pit::render::RenderInformation;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::material::Material;
use bottomless_pit::input::MouseKey;
use bottomless_pit::vec2;
use bottomless_pit::vectors::Vec2;
use utils::collision;

use crate::editor::EditorContext;
use crate::level::Platform;

pub trait Tool {
    fn on_click(&mut self, mouse_pos: Vec2<f32>, editor: &mut EditorContext);
    fn on_mouse_release(&mut self, mouse_pos: Vec2<f32>, editor: &mut EditorContext);
    fn can_switch(&self) -> bool;
    fn update(&mut self, engine: &mut Engine, editor: &mut EditorContext);
    fn init(&mut self, _editor: &mut EditorContext) {}
    fn draw(&self, material: &mut Material, editor: &EditorContext, renderer: &mut RenderInformation);
}

#[derive(Debug)]
pub struct PlatformTool {
    mouse_pressed_pos: Vec2<f32>,
    mouse_down: bool,
    preview_platform: Option<Platform>,
}

impl PlatformTool {
    pub fn new() -> Self {
        Self {
            mouse_pressed_pos: vec2!(0.0),
            mouse_down: false,
            preview_platform: None,
        }
    }
}

impl Tool for PlatformTool {
    fn on_click(&mut self, mouse_pos: Vec2<f32>, _: &mut EditorContext) {
        self.mouse_pressed_pos = mouse_pos;
    }

    fn on_mouse_release(&mut self, mouse_pos: Vec2<f32>, editor: &mut EditorContext) {
        let p = self.preview_platform.take().unwrap();
        let delta = self.mouse_pressed_pos - mouse_pos;
        let level = editor.get_mut_level();

        if delta.x != 0.0 && delta.y != 0.0 {
            level.add_platform(p);
        }
    }

    fn can_switch(&self) -> bool {
        self.preview_platform.is_none()
    }

    fn init(&mut self, editor: &mut EditorContext) {
        editor.selection = vec![];
    }

    fn update(&mut self, engine: &mut Engine, _: &mut EditorContext) {
        self.mouse_down = engine.is_mouse_key_down(MouseKey::Left);

        if self.mouse_down {
            self.preview_platform = Some(Platform::from_corners(self.mouse_pressed_pos, engine.get_mouse_position()));
        }
    }

    fn draw(&self, material: &mut Material, _: &EditorContext, renderer: &mut RenderInformation) {
        if let Some(p) = &self.preview_platform {
            p.draw(material, renderer);
        }
    }
}

#[derive(Debug)]
pub struct Selector {
    mouse_pressed_pos: Vec2<f32>,
    mouse_current_pos: Vec2<f32>,
    mouse_down: bool,
}

impl Selector {
    pub fn new() -> Self {
        Self {
            mouse_pressed_pos: vec2!(0.0),
            mouse_current_pos: vec2!(0.0),
            mouse_down: false,
        }
    }
}

impl Tool for Selector {
    fn update(&mut self, engine: &mut Engine, _: &mut EditorContext) {
        self.mouse_current_pos = engine.get_mouse_position();
    }

    fn on_click(&mut self, mouse_pos: Vec2<f32>, _: &mut EditorContext) {
        self.mouse_pressed_pos = mouse_pos;
        self.mouse_down = true;
    }

    fn can_switch(&self) -> bool {
        true
    }

    fn on_mouse_release(&mut self, _: Vec2<f32>, editor: &mut EditorContext) {
        self.mouse_down = false;

        let size = self.mouse_current_pos - self.mouse_pressed_pos;
        let size = vec2!(size.x.abs(), size.y.abs());
        let rect_start = vec2!(self.mouse_current_pos.x.min(self.mouse_pressed_pos.x), self.mouse_current_pos.y.min(self.mouse_pressed_pos.y));

        editor.selection = editor
            .get_level()
            .get_platforms()
            .iter()
            .enumerate()
            .filter_map(|(idx, p)| {
                if collision::rect_in_rect(p.pos, p.size, rect_start, size) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();
    }

    fn draw(&self, material: &mut Material, _: &EditorContext, renderer: &mut RenderInformation) {
        let thickness = 1.0_f32;

        if self.mouse_down {
            let size = self.mouse_current_pos - self.mouse_pressed_pos;

            let (x_start, x_end, x_mult) = if size.x.is_sign_negative() {
                (self.mouse_current_pos.x, self.mouse_pressed_pos.x, -1.0)
            } else {
                (self.mouse_pressed_pos.x, self.mouse_current_pos.x, 1.0)
            };

            let (y_start, y_end, y_mult) = if size.y.is_sign_negative() {
                (self.mouse_current_pos.y, self.mouse_pressed_pos.y, -1.0)
            } else {
                (self.mouse_pressed_pos.y, self.mouse_current_pos.y, 1.0)
            };

            material.add_rectangle(vec2!(x_start, y_start), vec2!(size.x * x_mult, thickness), Colour::RED, &renderer); 
            material.add_rectangle(vec2!(x_start, y_start), vec2!(thickness, size.y * y_mult), Colour::RED, &renderer);
            material.add_rectangle(vec2!(x_end, y_start), vec2!(thickness, (size.y * y_mult) + thickness), Colour::RED, &renderer);
            material.add_rectangle(vec2!(x_start, y_end), vec2!((size.x * x_mult) + thickness, thickness), Colour::RED, &renderer);         
        }
    }
}

#[derive(Debug)]
pub struct MoveTool {
    last_recorded_mouse: Vec2<f32>,
    total_move_delta: Vec2<f32>,
    mouse_down: bool,
}

impl MoveTool {
    pub fn new() -> Self {
        Self {
            last_recorded_mouse: vec2!(0.0),
            total_move_delta: vec2!(0.0),
            mouse_down: false,
        }
    }
}

impl Tool for MoveTool {
    fn on_click(&mut self, mouse_pos: Vec2<f32>, _: &mut EditorContext) {
        self.last_recorded_mouse = mouse_pos;
        self.mouse_down = true;
    }

    fn on_mouse_release(&mut self, _: Vec2<f32>, editor: &mut EditorContext) {
        editor.get_mut_level().move_selected_platforms(self.total_move_delta);
        self.total_move_delta = vec2!(0.0);
        self.mouse_down = false;

    }

    fn init(&mut self, _editor: &mut EditorContext) {
        
    }

    fn update(&mut self, engine: &mut Engine, _: &mut EditorContext) {
        let new_mouse_pos = engine.get_mouse_position();

        if self.mouse_down {
            let delta = new_mouse_pos - self.last_recorded_mouse;
            self.total_move_delta += delta;
            self.last_recorded_mouse = new_mouse_pos;
        }
        
    }

    fn can_switch(&self) -> bool {
        !self.mouse_down
    }

    fn draw(&self, material: &mut Material, editor: &EditorContext, renderer: &mut RenderInformation) {
        if self.mouse_down {
            editor
                .get_level()
                .get_platforms()
                .iter()
                .enumerate()
                .filter(|(idx, _)| editor.selection.contains(idx))
                .map(|(_, p)| (p.pos + self.total_move_delta, p.size))
                .for_each(|(pos, size)| material.add_rectangle(pos, size, Colour::from_rgba(255.0, 255.0, 255.0, 0.5), &renderer));
        }
    }
}