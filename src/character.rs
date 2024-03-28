use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::input::Key;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::render::RenderInformation;
use bottomless_pit::texture::{SamplerType, Texture};
use bottomless_pit::vec2;
use bottomless_pit::vectors::Vec2;

use crate::collision::{self, line_in_rect, point_in_rect};
use crate::level::{Level, Platform};

const PLAYER_ACCELERATION: f32 = 60.0;
const PLAYER_DECLERATION: f32 = 100.0;
const PLAYER_TURN_SPEED: f32 = 250.0;
const PLAYER_MAX_SPEED: f32 = 120.0;
const PLAYER_FALL_ACCELERATION: f32 = 80.0;
const MAX_FALL_SPEED: f32 = 200.0;
const PLAYER_SIZE: Vec2<f32> = vec2!(96.0, 114.0);

pub struct Character {
    pos: Vec2<f32>,
    speed: Vec2<f32>,
    size: Vec2<f32>,
    fastest_y: f32,
    material: Material,
    state: PlayerState,
}

impl Character {
    pub fn new(engine: &mut Engine) -> Self {
        let texture = Texture::new_with_sampler(engine, "assets/shork.png", SamplerType::NearestNeighbor);

        let material = MaterialBuilder::new()
            .add_texture(texture)
            .build(engine);

        Self {
            pos: Vec2{x: 0.0, y: -200.0},
            speed: Vec2{x: 0.0, y: 0.0},
            size: PLAYER_SIZE,
            material,
            fastest_y: 0.0,
            state: PlayerState::Falling,
        }
    }

    pub fn get_pos(&self) -> Vec2<f32> {
        self.pos
    }

    pub fn get_cetner(&self) -> Vec2<f32> {
        vec2!(self.pos.x + self.size.x / 2.0, self.pos.y + self.size.y / 2.0)
    } 

    pub fn get_size(&self) -> Vec2<f32> {
        self.size
    }

    pub fn get_speed(&self) -> Vec2<f32> {
        self.speed
    }

    pub fn get_state(&self) -> PlayerState {
        self.state
    }

    pub fn update(&mut self, dt: f32, engine: &mut Engine, level: &Level) {
        // if self.grounded {
        //     self.grounded_movement(dt, engine);
        // } else {
        //     self.air_movment(dt);
        // }
        match self.state {
            PlayerState::Grounded => self.grounded_movement(dt, engine),
            PlayerState::Falling => self.air_movment(dt),
            _ => unimplemented!(),
        }

        self.pos += self.speed.scale(dt);

        let not_collided = level.get_platforms().iter().all(|p| !self.bottom_collision(p));
        if not_collided {
            self.state = PlayerState::Falling;
        }


        // print!("{esc}c", esc = 27 as char);
        // println!("speed: {:?}", self.speed);
        // println!("fastest y: {:?}", self.fastest_y);
        // println!("dt: {:.5}", dt);
        // println!("frame rate: {:.0}", engine.get_stable_fps());
        // println!("mouse over_player: {}", collision::point_in_rect(engine.get_mouse_position(), self.pos, self.size));
        // println!("player state: {:?}", self.state);
    }

    pub fn request_transition(&mut self, new_state: PlayerState, reason: TransReason) {
        match (self.state, new_state, reason) {
            (_, _, _) => todo!()
        }
    }

    fn grounded_movement(&mut self, dt: f32, engine: &mut Engine) {
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

        if engine.is_key_down(Key::Space) {
            self.speed.y = -100.0;
        }

        // caps both backwards and forwards speed
        self.speed.x = self.speed.x.min(PLAYER_MAX_SPEED);
        self.speed.x = self.speed.x.max(-PLAYER_MAX_SPEED);

        self.fastest_y = self.speed.y.max(self.fastest_y);
    }

    fn air_movment(&mut self, dt: f32) {
        self.speed.y += PLAYER_FALL_ACCELERATION * dt;
        self.speed.y = self.speed.y.min(MAX_FALL_SPEED);
    }

    fn bottom_collision(&mut self, platform: &Platform) -> bool {
        let bottom_left = point_in_rect(vec2!(self.pos.x, self.pos.y + self.size.y), platform.pos, platform.size);
        let bottom_right = point_in_rect(vec2!(self.pos.x + self.size.x, self.pos.y + self.size.y), platform.pos, platform.size);

        if bottom_left || bottom_right {
            self.state = PlayerState::Grounded;
            self.pos.y = platform.pos.y - self.size.y;
            self.speed.y = 0.0;
            true
        } else {
            false
        }
    }

    pub fn draw<'p, 'o>(&'o mut self, renderer: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        self.material.add_rectangle(self.pos, self.size, Colour::WHITE, &renderer);

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

#[derive(Clone, Copy, Debug)]
pub enum PlayerState {
    Grounded,
    Jumping,
    Falling,
}

#[derive(Clone, Copy, Debug)]
pub enum TransReason {
    GroudCollision,
    JumpStart,
    NothingBellow,
}