use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::input::Key;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::render::RenderInformation;
use bottomless_pit::vectors::Vec2;

const PLAYER_ACCELERATION: f32 = 60.0;
const PLAYER_DECLERATION: f32 = 100.0;
const PLAYER_TURN_SPEED: f32 = 250.0;
const PLAYER_MAX_SPEED: f32 = 120.0;
const PLAYER_FALL_ACCELERATION: f32 = 80.0;
const MAX_FALL_SPEED: f32 = 200.0;

pub struct Character {
    pos: Vec2<f32>,
    speed: Vec2<f32>,
    fastest_y: f32,
    material: Material,
    grounded: bool,
}

impl Character {
    pub fn new(engine: &mut Engine) -> Self {
        let material = MaterialBuilder::new().build(engine);

        Self {
            pos: Vec2{x: 0.0, y: 0.0},
            speed: Vec2{x: 0.0, y: 0.0},
            material,
            fastest_y: 0.0,
            grounded: false,
        }
    }

    pub fn update(&mut self, dt: f32, engine: &mut Engine) {
        let mut move_x: f32 = 0.0;

        if engine.is_key_down(Key::D) {
            move_x += 1.0;
        }

        if engine.is_key_down(Key::A) {
            move_x -= 1.0;
        }

        // if they dont have the same sign ur turning
        let max_speed = if move_x != 0.0 && self.speed.x.is_sign_positive() != move_x.is_sign_positive() {
            dt * PLAYER_TURN_SPEED
        } else {
            dt * PLAYER_ACCELERATION
        };

        if move_x == 0.0 {
            self.speed.x = move_towards(self.speed.x, 0.0, PLAYER_DECLERATION * dt);
        } else {
            self.speed.x = move_towards(self.speed.x, move_x * PLAYER_MAX_SPEED, max_speed);
        }

        // shoe in before solids/platforms are added
        if self.pos.y >= 600.0 - 80.0 {
            self.grounded = true;
            self.pos.y = 600.0 - 80.0;
        }

        if !self.grounded {
            self.speed.y += PLAYER_FALL_ACCELERATION * dt;
        } else {
            self.speed.y = 0.0;
        }

        // caps both backwards and forwards speed
        self.speed.x = self.speed.x.min(PLAYER_MAX_SPEED);
        self.speed.x = self.speed.x.max(-PLAYER_MAX_SPEED);

        self.speed.y = self.speed.y.min(MAX_FALL_SPEED);
        self.fastest_y = self.speed.y.max(self.fastest_y);

        self.pos += self.speed.scale(dt);

        // messed up on windows print!("{esc}c", esc = 27 as char);
        println!("speed: {:?}", self.speed);
        println!("fastest y: {:?}", self.fastest_y);
        println!("dt: {}", dt);
        println!("frame rate: {}", 1.0 / dt);
    }

    pub fn draw<'p, 'o>(&'o mut self, renderer: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        self.material.add_rectangle(self.pos, Vec2{x: 40.0, y: 80.0}, Colour::WHITE, &renderer);

        self.material.draw(renderer);
    }
}

// linear????
// need to integrate this somehow
fn move_towards(current: f32, target: f32, max_delta: f32) -> f32 {
    if (current-target).abs() <= max_delta {
        return target
    } else {
        let sign = if (target-current).is_sign_positive() {
            1.0
        } else {
            -1.0
        };

        return current + sign * max_delta;
    }
}