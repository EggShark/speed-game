mod character;

use character::Character;

use bottomless_pit::Game;
use bottomless_pit::engine_handle::{Engine, EngineBuilder};
use bottomless_pit::render::RenderInformation;

fn main() {
    let mut engine = EngineBuilder::new()
        .build()
        .unwrap();

    let game = SpeedGame::new(&mut engine);

    engine.run(game);
}

struct SpeedGame {
    player: Character
}

impl SpeedGame {
    pub fn new(engine: &mut Engine) -> Self {
        let player = Character::new(engine);

        Self {
            player,
        }
    }
}

impl Game for SpeedGame {
    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.get_frame_delta_time();

        println!("dt: {}", dt);

        self.player.update(dt, engine);
    }

    fn render<'pass, 'others>(&'others mut self, mut renderer: RenderInformation<'pass, 'others>) where 'others: 'pass {
        self.player.draw(&mut renderer);
    }
}