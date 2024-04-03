use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::input::Key;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::render::RenderInformation;
use bottomless_pit::texture::{SamplerType, Texture};
use bottomless_pit::vec2;
use bottomless_pit::vectors::Vec2;

use utils::collision::point_in_rect;
use level_editor::level::{Level, Platform};

const PLAYER_ACCELERATION: f32 = 190.0;
const PLAYER_DECLERATION: f32 = 100.0;
const PLAYER_TURN_SPEED: f32 = 250.0;
const PLAYER_MAX_SPEED: f32 = 200.0;
const PLAYER_MAX_AIRSPEED: f32 = 200.0;
const PLAYER_AIR_ACCEL: f32 = 145.0;
const PLAYER_AIR_DECEL: f32 = 50.0;
const PLAYER_AIR_TURN_SPEED: f32 = 150.0;

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
    friction: f32,
}

impl Character {
    pub fn new(engine: &mut Engine) -> Self {
        let texture = Texture::new_with_sampler(engine, "speed-game/assets/shork.png", SamplerType::NearestNeighbor);

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
            friction: 1.0,
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

    pub fn update(&mut self, dt: f32, engine: &Engine, level: &Level) {
        match self.state {
            PlayerState::Grounded => self.grounded_movement(dt, engine),
            PlayerState::Falling => self.air_movment(dt, engine),
            PlayerState::Jumping => self.jumping_movement(dt, engine),
            _ => unimplemented!(),
        }

        self.pos += self.speed.scale(dt);

        let not_collided = level.get_platforms().iter().all(|p| !self.bottom_collision(p));
        if not_collided && self.state != PlayerState::Falling {
            self.request_transition(PlayerState::Falling, TransReason::NothingBellow);
        }
    }

    pub fn request_transition(&mut self, new_state: PlayerState, reason: TransReason) {
        match (self.state, new_state, reason) {
            (PlayerState::Grounded, PlayerState::Grounded, _) => {},
            (PlayerState::Grounded, PlayerState::Jumping, TransReason::JumpStart) => {
                self.state = PlayerState::Jumping;
            },
            (PlayerState::Grounded, PlayerState::Falling, TransReason::NothingBellow) => {
                self.state = PlayerState::Falling;
            },
            (PlayerState::Jumping, PlayerState::Jumping, _) => {},
            (PlayerState::Jumping, PlayerState::Falling, TransReason::NothingBellow) => {
                if self.speed.y > 0.0 {
                    self.state = PlayerState::Falling;
                }
            },
            (PlayerState::Jumping, PlayerState::Grounded, TransReason::GroudCollision) => {
                self.state = PlayerState::Grounded;
            },
            (PlayerState::Falling, PlayerState::Falling, _) => {},
            (PlayerState::Falling, PlayerState::Grounded, TransReason::GroudCollision) => {
                self.state = PlayerState::Grounded;
            },
            (_, _, _) => todo!()
        }
    }

    fn grounded_movement(&mut self, dt: f32, engine: &Engine) {
        self.horizontal_movment(engine, dt, [PLAYER_TURN_SPEED, PLAYER_ACCELERATION, PLAYER_DECLERATION, PLAYER_MAX_SPEED]);

        if engine.is_key_down(Key::Space) {
            self.jump_action();
        }

        // caps both backwards and forwards speed
        self.speed.x = self.speed.x.min(PLAYER_MAX_SPEED);
        self.speed.x = self.speed.x.max(-PLAYER_MAX_SPEED);

        self.fastest_y = self.speed.y.max(self.fastest_y);
    }

    fn horizontal_movment(&mut self, engine: &Engine, dt: f32, constants: [f32; 4]) {
        let mut move_x: f32 = 0.0;

        if engine.is_key_down(Key::D) {
            move_x += 1.0;
        }

        if engine.is_key_down(Key::A) {
            move_x -= 1.0;
        }

        // if they dont have the same sign ur turning
        let max_speed = if move_x != 0.0 && self.speed.x.is_sign_positive() != move_x.is_sign_positive() {
            dt * constants[0]
        } else {
            dt * constants[1]
        };

        if move_x == 0.0 {
            self.speed.x = move_towards(self.speed.x, 0.0, constants[2] * dt);
        } else {
            self.speed.x = move_towards(self.speed.x, move_x * constants[3], max_speed);
        }
    }

    fn air_movment(&mut self, dt: f32, engine: &Engine) {
        self.speed.y += PLAYER_FALL_ACCELERATION * dt;
        self.speed.y = self.speed.y.min(MAX_FALL_SPEED);

        // if self.speed.y >= -20.0 && self.speed.y <= 20.0 {
        //     println!("top of jump?");
        // } else {
        //     println!("normal jump");
        // }
        self.horizontal_movment(engine, dt, [PLAYER_AIR_TURN_SPEED, PLAYER_AIR_ACCEL, PLAYER_AIR_DECEL, PLAYER_MAX_AIRSPEED])
    }

    fn jumping_movement(&mut self, dt: f32, engine: &Engine) {
        if engine.is_key_released(Key::Space) {
            self.speed.y = f32::min(self.speed.y + 40.0, 0.0);
        }

        self.air_movment(dt, engine);
    }

    fn jump_action(&mut self) {
        if self.state == PlayerState::Grounded {
            self.speed.y = -(2.0_f32 * PLAYER_FALL_ACCELERATION * 100.0).sqrt();
        }

        self.request_transition(PlayerState::Jumping, TransReason::JumpStart);
    }

    fn bottom_collision(&mut self, platform: &Platform) -> bool {
        let bottom_left = point_in_rect(vec2!(self.pos.x, self.pos.y + self.size.y), platform.pos, platform.size);
        let bottom_right = point_in_rect(vec2!(self.pos.x + self.size.x, self.pos.y + self.size.y), platform.pos, platform.size);

        if bottom_left || bottom_right {
            self.request_transition(PlayerState::Grounded, TransReason::GroudCollision);
            self.pos.y = platform.pos.y - self.size.y;
            self.speed.y = 0.0;
            self.friction = platform.friction;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlayerState {
    Grounded,
    Jumping,
    Falling,
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum TransReason {
    GroudCollision,
    JumpStart,
    NothingBellow,
}