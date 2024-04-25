use std::env;
use std::fmt::Debug;
use std::path::Path;

use bottomless_pit::input::{Key, MouseKey, ModifierKeys};
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::{Game, vec2};
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::render::RenderInformation;
use bottomless_pit::vectors::Vec2;
use utils::ui::button::{Button, CallBackButton};
use crate::level::{Level, Platform};
use crate::tools::{MoveTool, PlatformTool, Selector, Tool};


#[derive(Debug)]
struct EditorWithState<S> {
    state: S,
    editor_mat: Material,
}

impl EditorWithState<Editing> {
    fn update(&mut self, engine: &mut Engine) -> Event {
        let mouse_pos = engine.get_mouse_position();

        if self.state.current_tool.can_switch() {
            self.change_tool(engine);
        }

        if engine.is_mouse_key_pressed(MouseKey::Left) {
            self.state.current_tool.on_click(mouse_pos, &mut self.state.context);
        } else if engine.is_mouse_key_released(MouseKey::Left) {
            self.state.current_tool.on_mouse_release(mouse_pos, &mut self.state.context);
        }

        self.state.current_tool.update(engine, &mut self.state.context);

        if engine.check_modifiers(ModifierKeys::Ctrl) && engine.is_key_pressed(Key::S) {
            let working_dir = env::current_dir().unwrap();

            let path = rfd::FileDialog::new()
                .add_filter("Speed Game Level Data", &["sgld"])
                .set_directory(working_dir)
                .set_file_name("out.sgld")
                .save_file();

            if let Some(p) = path {
                self.state.context.write_level_to_file(p).unwrap();
            }
        }


        Event::None
    }

    fn change_tool(&mut self, engine: &mut Engine) {
        if engine.is_key_pressed(Key::S) {
            self.state.current_tool = Box::new(Selector::new());
        } else if engine.is_key_pressed(Key::P) {
            self.state.current_tool = Box::new(PlatformTool::new());
        } else if engine.is_key_pressed(Key::M) {
            self.state.current_tool = Box::new(MoveTool::new());
        }
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>) where 'others: 'pass {
        self.state.current_tool.draw(&mut self.editor_mat, &mut self.state.context, &mut renderer);
        self.editor_mat.draw(&mut renderer);

        self.state.context.render(renderer);
    }
}

impl EditorWithState<Menu> {
    fn new(engine: &mut Engine) -> Self {
        let editor_mat = MaterialBuilder::new().build(engine);

        Self {
            editor_mat,
            state: Menu::new(),
        }
    }

    fn update(&mut self, engine: &mut Engine) -> Event {
        let mouse_pos = engine.get_mouse_position();
        let mut event_to_transmit = Event::None;
        
        let _ = self.state.quit_button.update(mouse_pos, &engine, &mut event_to_transmit) ||
            self.check_buttons(mouse_pos, engine, &mut event_to_transmit);

        // if engine.is_key_down(Key::M) {
        //     let level_mat = MaterialBuilder::new().build(engine);
        //     Event::OpenLevel(Level::new(vec![Platform::new(vec2!(0.0), vec2!(100.0))], level_mat))
        // } else {
        //     Event::None
        // }

        event_to_transmit
    }

    fn check_buttons(&mut self, mouse_pos: Vec2<f32>, engine: &mut Engine, event: &mut Event) -> bool {
        if self.state.to_level.was_clicked(mouse_pos, engine) {
            let material = MaterialBuilder::new().build(engine);
            *event = Event::OpenLevel(Level::new(vec![], material));
            true
        } else {
            false
        }
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>) where 'others: 'pass {
        self.state.quit_button.render(&mut self.editor_mat, &renderer);
        self.state.to_level.render(&mut self.editor_mat, &renderer);

        self.editor_mat.draw(&mut renderer);
    }
}

trait CoolTool: Tool + Debug {}
impl CoolTool for Selector {}
impl CoolTool for PlatformTool {}
impl CoolTool for MoveTool {}

struct Menu {
    quit_button: CallBackButton<Event>,
    to_level: Button,
}

impl Menu {
    fn new() -> Self {
        let func = |event: &mut Event| { *event = Event::Quit };

        let quit_button = CallBackButton::new(vec2!(100.0), vec2!(100.0), func);
        let to_level = Button::new(vec2!(100.0), vec2!(250.0, 100.0));

        Self {
            quit_button,
            to_level,
        }
    }
}

impl Debug for Menu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Menu")
            .field("quit: Button", &self.quit_button)
            .finish()
    }
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
    Quit,
    None,
}

#[derive(Debug)]
enum EditorState {
    Menu(EditorWithState<Menu>),
    Editing(EditorWithState<Editing>),
    Failure(String),
    Quiting,
    Dummy,
}

impl EditorState {
    fn new(engine: &mut Engine) -> Self {
        let menu = EditorWithState::new(engine);
        Self::Menu(menu)
    }

    fn next(self, event: Event) -> Self {
        match (self, event) {
            (Self::Menu(m), Event::OpenLevel(l)) => Self::Editing((l, m).into()),
            (_, Event::Quit) => EditorState::Quiting,
            (s, Event::None) => s,
            (s, e) => Self::Failure(format!("Bad Combo: {:?}, {:?}", s, e)), 
        }
    }

    fn update(&mut self, engine: &mut Engine) -> Event {
        match self {
            Self::Menu(m) => m.update(engine),
            Self::Editing(e) => e.update(engine),
            Self::Failure(s) => panic!("{}", s),
            Self::Dummy | Self::Quiting => unreachable!(),
        }
    }

    fn render<'pass, 'others>(&'others mut self, renderer: RenderInformation<'pass, 'others>) {
        match self {
            Self::Menu(m) => m.render(renderer),
            Self::Editing(e) => e.render(renderer),
            Self::Failure(s) => panic!("{}", s),
            Self::Quiting => {},
            Self::Dummy => unreachable!(),
        }
    }
}

impl From<(Level, EditorWithState<Menu>)> for EditorWithState<Editing> {
    fn from((level, value): (Level, EditorWithState<Menu>)) -> Self {
        Self {
            state: Editing {
                context: EditorContext::new(level),
                current_tool: Box::new(PlatformTool::new()),
            },
            editor_mat: value.editor_mat,
        }
    }
}

impl From<EditorWithState<Editing>> for EditorWithState<Menu> {
    fn from(value: EditorWithState<Editing>) -> Self {
        Self {
            state: Menu::new(),
            editor_mat: value.editor_mat,
        }
    }
}

pub struct MainEditor {
    inner: EditorState,
}

impl MainEditor {
    pub fn new(engine: &mut Engine) -> Self {
        let inner = EditorState::new(engine);
        Self {
            inner
        }
    }
}

impl Game for MainEditor {
    fn update(&mut self, engine_handle: &mut Engine) {
        let mut dummy = EditorState::Dummy;
        std::mem::swap(&mut dummy, &mut self.inner);
        let event = dummy.update(engine_handle);
        dummy = dummy.next(event);
        std::mem::swap(&mut dummy, &mut self.inner);

        if matches!(self.inner, EditorState::Quiting) {
            engine_handle.close();
        }
    }

    fn render<'pass, 'others>(&'others mut self, renderer: RenderInformation<'pass, 'others>)
        where
            'others: 'pass {
        self.inner.render(renderer)
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

    pub fn move_selected_platforms(&mut self, delta: Vec2<f32>) {
        self.level.move_selected_platforms(&self.selection, delta);
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

