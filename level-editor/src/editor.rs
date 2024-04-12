use std::env;
use std::path::Path;

use bottomless_pit::input::{Key, MouseKey, ModifierKeys};
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::{engine_handle, Game};
use bottomless_pit::engine_handle::{Engine, EngineBuilder};
use bottomless_pit::render::RenderInformation;
use crate::level::{Level, Platform};
use crate::tools::{PlatformTool, Selector, Tool};


pub struct Editor {
    context: EditorContext,
    current_tool: Box<dyn Tool>,
}

impl Editor {
    pub fn new(mat: Material) -> Self {
        let level = Level::new(vec![], mat);
        let context = EditorContext::new(level);


        Self {
            context,
            current_tool: Box::new(PlatformTool::new()),
        }
    }

    fn change_tool(&mut self, engine: &mut Engine) {
        if engine.is_key_pressed(Key::S) {
            self.current_tool = Box::new(Selector::new());
        } else if engine.is_key_pressed(Key::P) {
            self.current_tool = Box::new(PlatformTool::new());
        }
    }
}

impl Game for Editor {
    fn update(&mut self, engine_handle: &mut Engine) {
        let mouse_pos = engine_handle.get_mouse_position();

        if self.current_tool.can_switch() {
            self.change_tool(engine_handle);
        }

        if engine_handle.is_mouse_key_pressed(MouseKey::Left) {
            self.current_tool.on_click(mouse_pos, &mut self.context);
        } else if engine_handle.is_mouse_key_released(MouseKey::Left) {
            self.current_tool.on_mouse_release(mouse_pos, &mut self.context);
        }

        self.current_tool.update(engine_handle, &mut self.context);

        if engine_handle.check_modifers(ModifierKeys::Ctrl) && engine_handle.is_key_pressed(Key::S) {
            let working_dir = env::current_dir().unwrap();

            let path = rfd::FileDialog::new()
                .add_filter("Speed Game Level Data", &["sgld"])
                .set_directory(working_dir)
                .set_file_name("out.sgld")
                .save_file();

            if let Some(p) = path {
                self.context.write_level_to_file(p).unwrap();
            }
        }
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>)
        where
            'others: 'pass {

        self.current_tool.draw(self.context.get_level_platform_mat(), &mut renderer);
        self.context.render(renderer);
    }
}

/// anything tools need to change pretty much
pub(crate) struct EditorContext {
    level: Level,
    pub(crate) selection: Vec<usize>,
}

impl EditorContext {
    fn new(level: Level) -> Self {
        Self {
            level,
            selection: vec![],
        }
    }

    fn write_level_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        self.level.write_to_file(path)
    }

    fn get_level_platform_mat(&mut self) -> &mut Material {
        self.level.get_platform_mat()
    }

    pub fn get_mut_level(&mut self) -> &mut Level {
        &mut self.level
    }

    pub fn get_level(&self) -> &Level {
        &self.level
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>)
    where
        'others: 'pass {

        self.level.draw(&mut renderer);
    }
}