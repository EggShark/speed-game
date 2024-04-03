use bottomless_pit::Game;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::render::RenderInformation;

fn main() {
    println!("Hello, world!");
}


struct Editor {

}

impl Game for Editor {
    fn update(&mut self, engine_handle: &mut Engine) {
        
    }

    fn render<'pass, 'others>(&'others mut self, mut render_handle: RenderInformation<'pass, 'others>)
        where
            'others: 'pass {
        
    }
}