use bottomless_pit::input::Key;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::vec2;
use bottomless_pit::vectors::Vec2;
use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::text::TextMaterial;
use bottomless_pit::render::RenderInformation;

use crate::character::Character;

pub struct DebugText {
    text_mat: TextMaterial,
    material: Material,
    text: String,
    line_poses: Vec<usize>,
    active: bool,
}

impl DebugText {
    pub fn new(engine: &mut Engine) -> Self {
        let text = String::from("pos: {}\nspeed: {}\ndt: {}\nframerate: {}\nplayerstate");

        let line_numbers = text
            .bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'\n')
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();

        let material = MaterialBuilder::new().build(engine);

        let text_mat = TextMaterial::new(
            &text,
            Colour::WHITE,
            10.0,
            12.0,
            engine
        );

        Self {
            text_mat,
            material,
            text,
            line_poses: line_numbers,
            active: false,
        }
    }

    pub fn update_player_info(&mut self, player: &Character) {
        self.replace_line(0, &format!("pos: {:?}", player.get_pos()));
        self.replace_line(1, &format!("speed: {:?}", player.get_speed()));
        self.replace_line(4, &format!("playerstate: {:?}", player.get_state()))
    }

    pub fn update_engine_info(&mut self, engine: &Engine, dt: f32) {
        self.replace_line(2, &format!("dt: {}", dt));
        self.replace_line(3, &format!("frame_rate: {:.2}", engine.get_stable_fps()));
    }

    fn replace_line(&mut self, idx: usize, text: &str) {
        let end = if idx >= self.line_poses.len() {
            self.text.len()
        } else {
            self.line_poses[idx]
        };

        let start = if idx == 0 {
            0
        } else {
            self.line_poses[idx - 1] + 1
        };

        self.text.replace_range(start..end, text);
        self.update_line_numbers();
    }

    fn update_line_numbers(&mut self) {
        self.line_poses = self.text
            .bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'\n')
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();
    }

    pub fn prepare(&mut self, engine: &mut Engine) {
        if engine.is_key_pressed(Key::F3) || engine.is_key_pressed(Key::P) {
            self.active = !self.active;
        }

        if self.active {
            self.text_mat.set_text(&self.text, Colour::WHITE, engine);

            self.text_mat.prepare(engine);
        }
    }

    pub fn draw<'p, 'o>(&'o mut self, renderer: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        if self.active {
            renderer.reset_camera();

            let size = self.text_mat.get_measurements();
            self.material.add_rectangle(vec2!(0.0), vec2!(size.x as f32, size.y as f32), Colour::BLACK, renderer);
            self.text_mat.add_instance(vec2!(0.0), Colour::WHITE, renderer);
    
            self.material.draw(renderer);
            self.text_mat.draw(renderer);
        }
    }
}