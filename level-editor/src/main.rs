use std::fmt::Debug;

use bottomless_pit::colour::Colour;
use bottomless_pit::input::MouseKey;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::Game;
use bottomless_pit::vec2;
use bottomless_pit::vectors::Vec2;
use bottomless_pit::engine_handle::{Engine, EngineBuilder};
use bottomless_pit::render::RenderInformation;
use level_editor::level::{Level, Platform};
use level_editor::tools::{PlatformTool, Selector, Tool};


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
    material: Material,
    level: Level,
    preview_platform: Option<Platform>,
    selected_items: Vec<usize>,
    current_tool: Box<dyn Tool>,
}

impl Editor {
    pub fn new(mat: Material) -> Self {
        Self {
            material: mat,
            level: Level::new(vec![]),
            preview_platform: None,
            selected_items: Vec::new(),
            current_tool: Box::new(PlatformTool::new()),
        }
    }
}

impl Game for Editor {
    fn update(&mut self, engine_handle: &mut Engine) {
        let mouse_pos = engine_handle.get_mouse_position();

        if engine_handle.is_mouse_key_pressed(MouseKey::Left) {
            self.current_tool.on_click(mouse_pos, &mut self.level);
        } else if engine_handle.is_mouse_key_released(MouseKey::Left) {
            self.current_tool.on_mouse_release(mouse_pos, &mut self.level);
        }

        self.current_tool.update(engine_handle, &mut self.level);
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>)
        where
            'others: 'pass {

        self.current_tool.draw(&mut self.material, &mut renderer);
        self.level.draw(&mut self.material, &mut renderer);
    }
}