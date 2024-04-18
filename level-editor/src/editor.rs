use std::env;
use std::fmt::Debug;
use std::path::Path;

use bottomless_pit::input::{Key, MouseKey, ModifierKeys};
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::Game;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::render::RenderInformation;
use bottomless_pit::vectors::Vec2;
use crate::level::Level;
use crate::tools::{MoveTool, PlatformTool, Selector, Tool};


pub struct Editor {
    context: EditorContext,
    current_tool: Box<dyn Tool>,
    editor_mat: Material,
}

impl Editor {
    pub fn new(mat: Material, engine: &mut Engine) -> Self {
        let level = Level::new(vec![], mat);
        let context = EditorContext::new(level);
        let editor_mat = MaterialBuilder::new().build(engine);

        Self {
            context,
            current_tool: Box::new(PlatformTool::new()),
            editor_mat,
        }
    }

    fn change_tool(&mut self, engine: &mut Engine) {
        if engine.is_key_pressed(Key::S) {
            self.current_tool = Box::new(Selector::new());
        } else if engine.is_key_pressed(Key::P) {
            self.current_tool = Box::new(PlatformTool::new());
        } else if engine.is_key_down(Key::M) {
            self.current_tool = Box::new(MoveTool::new());
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

        if engine_handle.check_modifiers(ModifierKeys::Ctrl) && engine_handle.is_key_pressed(Key::S) {
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

        self.current_tool.draw(&mut self.editor_mat, &mut self.context, &mut renderer);
        self.editor_mat.draw(&mut renderer);

        self.context.render(renderer);

    }
}

/// anything tools need to change pretty much
#[derive(Debug)]
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


#[derive(Debug)]
struct EditorWithState<S> {
    state: S,
    data: usize,
    editor_mat: Material,
}

impl EditorWithState<Editing> {
    fn update(&mut self, engine: &mut Engine) -> Event {


        Event::None
    }

    fn render<'pass, 'others>(&'others mut self, render_handle: RenderInformation<'pass, 'others>) {
        
    }
}

impl EditorWithState<Menu> {
    fn update(&mut self, engine: &mut Engine) -> Event {


        Event::None
    }

    fn render<'pass, 'others>(&'others mut self, render_handle: RenderInformation<'pass, 'others>) {

    }
}

trait CoolTool: Tool + Debug {}
impl CoolTool for Selector {}

#[derive(Debug)]
struct Menu {
    button_pos: Vec<Vec2<f32>>,
}

#[derive(Debug)]
struct Editing {
    context: EditorContext,
    current_tool: Box<dyn CoolTool>,
}

#[derive(Debug)]
enum Event {
    OpenLevel(Level),
    BackToMenu,
    None,
}

#[derive(Debug)]
enum EditorState {
    Menu(EditorWithState<Menu>),
    Editing(EditorWithState<Editing>),
    Failure(String),
    Dummy,
}

impl EditorState {
    fn next(self, event: Event) -> Self {
        match (self, event) {
            (Self::Menu(m), Event::OpenLevel(l)) => Self::Editing((l, m).into()),
            (s, Event::None) => s,
            (s, e) => Self::Failure(format!("Bad Combo: {:?}, {:?}", s, e)), 
        }
    }

    fn update(&mut self, engine: &mut Engine) -> Event {
        match self {
            Self::Menu(m) => m.update(engine),
            Self::Editing(e) => e.update(engine),
            Self::Dummy => unreachable!(),
            Self::Failure(s) => panic!("{}", s),
        }
    }

    fn render<'pass, 'others>(&'others mut self, renderer: RenderInformation<'pass, 'others>) {
        match self {
            Self::Menu(m) => m.render(renderer),
            Self::Editing(e) => e.render(renderer),
            Self::Dummy => unreachable!(),
            Self::Failure(s) => panic!("{}", s),
        }
    }
}

impl From<(Level, EditorWithState<Menu>)> for EditorWithState<Editing> {
    fn from((level, value): (Level, EditorWithState<Menu>)) -> Self {
        Self {
            state: Editing {
                context: EditorContext::new(level),
                current_tool: Box::new(Selector::new()),
            },
            data: value.data,
            editor_mat: value.editor_mat,
        }
    }
}

impl From<EditorWithState<Editing>> for EditorWithState<Menu> {
    fn from(value: EditorWithState<Editing>) -> Self {
        Self {
            data: value.data,
            state: Menu{ button_pos: Vec::new(),},
            editor_mat: value.editor_mat,
        }
    }
}

struct MainEditor {
    inner: EditorState,
}

impl Game for MainEditor {
    fn update(&mut self, engine_handle: &mut Engine) {
        let mut dummy = EditorState::Dummy;
        std::mem::swap(&mut dummy, &mut self.inner);
        let event = dummy.update(engine_handle);
        dummy = dummy.next(event);
        std::mem::swap(&mut dummy, &mut self.inner);
    }

    fn render<'pass, 'others>(&'others mut self, renderer: RenderInformation<'pass, 'others>)
        where
            'others: 'pass {
        self.inner.render(renderer)
    }
}