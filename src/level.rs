use bottomless_pit::colour::Colour;
use bottomless_pit::material::Material;
use bottomless_pit::render::RenderInformation;
use bottomless_pit::vectors::Vec2;

use crate::collision;

pub struct Level {
    platforms: Vec<Platform>,
    platform_material: Material,
}

impl Level {
    pub fn new(platforms: Vec<Platform>, platform_material: Material) -> Self {
        Self {
            platforms,
            platform_material,
        }
    }

    pub fn get_platforms(&self) -> &[Platform] {
        &self.platforms
    }

    pub fn draw<'p, 'o>(&'o mut self, renderer: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        for platform in &self.platforms {
            platform.draw(&mut self.platform_material, renderer)
        }

        self.platform_material.draw(renderer);
    }
}

pub struct Platform {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
}

impl Platform {
    pub fn new(pos: Vec2<f32>, size: Vec2<f32>) -> Self {
        Self {
            pos,
            size,
        }
    }

    pub fn check_collision(&self, other_pos: Vec2<f32>, other_size: Vec2<f32>) -> bool {
        collision::rect_in_rect(self.pos, self.size, other_pos, other_size)
    }

    fn draw(&self, mat: &mut Material, renderer: &RenderInformation) {
        mat.add_rectangle(self.pos, self.size, Colour::WHITE, renderer);
    }
}