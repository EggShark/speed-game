use bottomless_pit::Game;
use bottomless_pit::engine_handle::{Engine, EngineBuilder};
use bottomless_pit::render::RenderInformation;

fn main() {
    let mut engine = EngineBuilder::new()
        .build()
        .unwrap();

    let game = SpeedGame::new();

    engine.run(game);
}



struct SpeedGame {

}

impl SpeedGame {
    pub fn new() -> Self {
        Self{}
    }
}

impl Game for SpeedGame {
    fn update(&mut self, engine_handle: &mut Engine) {
        
    }

    fn render<'pass, 'others>(&'others mut self, mut render_handle: RenderInformation<'pass, 'others>) where 'others: 'pass {
        
    }
}
