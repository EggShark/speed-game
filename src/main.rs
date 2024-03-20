mod character;
mod collision;
mod level;

use bottomless_pit::material::MaterialBuilder;
use character::Character;
use level::{Level, Platform};

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
}

impl SpeedGame {
    pub fn new(engine: &mut Engine) -> Self {
        let player = Character::new(engine);

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
        }
    }
}

impl Game for SpeedGame {
    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.get_frame_delta_time();

        println!("dt: {}", dt);

        self.player.update(dt, engine, &self.current_level);
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>) where 'others: 'pass {
        self.current_level.draw(&mut renderer);

        self.player.draw(&mut renderer);
    }
}