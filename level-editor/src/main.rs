
use std::env;
use std::path::PathBuf;

use bottomless_pit::input::{Key, MouseKey, ModifierKeys};
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::Game;
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
    level: Level,
    current_tool: Box<dyn Tool>,
}

impl Editor {
    pub fn new(mat: Material) -> Self {
        Self {
            level: Level::new(vec![], mat),
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

        if engine_handle.check_modifers(ModifierKeys::Ctrl) && engine_handle.is_key_pressed(Key::S) {
            let mut working_dir = env::current_dir().unwrap();
            working_dir.push(PathBuf::from("out.sgld"));

            let path = rfd::FileDialog::new()
                .add_filter("Speed Game Level Data", &["sgld"])
                .set_directory(working_dir)
                .set_file_name("out.sgld")
                .save_file();

            if let Some(p) = path {
                self.level.write_to_file(p).unwrap();
            }
        }
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>)
        where
            'others: 'pass {

        self.current_tool.draw(self.level.get_platform_mat(), &mut renderer);
        self.level.draw(&mut renderer);
    }
}