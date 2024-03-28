mod character;
mod collision;
mod debug;
mod level;

use character::Character;
use level::{Level, Platform};
use debug::DebugText;

use bottomless_pit::camera::Camera;
use bottomless_pit::material::MaterialBuilder;
use bottomless_pit::{vec2, Game};
use bottomless_pit::vectors::Vec2;
use bottomless_pit::engine_handle::{Engine, EngineBuilder};
use bottomless_pit::render::RenderInformation;


fn main() {
    let mut engine = EngineBuilder::new()
        .with_resolution((600, 600))
        .build()
        .unwrap();

    let game = SpeedGame::new(&mut engine);

    engine.run(game);
}

struct SpeedGame {
    player: Character,
    current_level: Level,
    camera: Camera,
    debug: DebugText,
}

impl SpeedGame {
    pub fn new(engine: &mut Engine) -> Self {
        let player = Character::new(engine);
        let camera = Camera::new(engine);
        let debug = DebugText::new(engine);

        let platform_material = MaterialBuilder::new().build(engine);
        let current_level = Level::new(
            vec![
                Platform::new(vec2!(10.0, 200.0), vec2!(300.0, 100.0)),
                Platform::new(vec2!(0.0, 600.0), vec2!(600.0, 50.0)),
            ],
            platform_material,
        );

        Self {
            player,
            current_level,
            camera,
            debug,
        }
    }
}

impl Game for SpeedGame {
    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.get_frame_delta_time();

        self.player.update(dt, engine, &self.current_level);
        self.camera.center = self.player.get_cetner();
        self.debug.update_player_info(&self.player);
        self.debug.update_engine_info(engine, dt);
        self.debug.prepare(engine);
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>) where 'others: 'pass {
        self.camera.set_active(&mut renderer);
        self.current_level.draw(&mut renderer);
        self.player.draw(&mut renderer);

        self.debug.draw(&mut renderer);
    }
}