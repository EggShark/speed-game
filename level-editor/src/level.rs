use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufReader, Read, Write};

use bottomless_pit::colour::Colour;
use bottomless_pit::material::Material;
use bottomless_pit::render::RenderInformation;
use bottomless_pit::vec2;
use bottomless_pit::vectors::Vec2;

use utils::collision;

// SGLD in bytes
const FILE_HEADER: [u8; 4] = [115, 103, 108, 100];


pub struct Level {
    platform_material: Material,
    inner: InnerLevel
}

impl Level {
    pub fn new(platforms: Vec<Platform>, platform_material: Material) -> Self {
        Self {
            platform_material,
            inner: InnerLevel::new(platforms),
        }
    }

    pub fn get_platforms(&self) -> &[Platform] {
        self.inner.get_platforms()
    }

    pub fn draw<'p, 'o>(&'o mut self, renderer: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        self.inner.draw(&mut self.platform_material, renderer);
    } 

    pub fn get_platform_mat(&mut self) -> &mut Material {
        &mut self.platform_material
    }

    pub(crate) fn add_platform(&mut self, platform: Platform) {
        self.inner.add_platform(platform);
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        self.inner.write_to_file(path)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct InnerLevel {
    platforms: Vec<Platform>,
    player_start: Vec2<f32>,
}

impl InnerLevel {
    pub fn new(platforms: Vec<Platform>) -> Self {
        Self {
            platforms,
            player_start: Vec2 { x: 0.0, y: 0.0},
        }
    }

    pub fn get_platforms(&self) -> &[Platform] {
        &self.platforms
    }

    pub fn draw<'p, 'o>(&'o mut self, platform_material: &'o mut Material, renderer: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        for platform in &self.platforms {
            platform.draw(platform_material, renderer)
        }

        platform_material.draw(renderer);
    }

    pub(crate) fn add_platform(&mut self, platform: Platform) {
        self.platforms.push(platform);
    }

    pub(crate) fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let mut platform_bytes = self
            .platforms
            .iter()
            .map(|p| p.to_bytes())
            .flatten()
            .collect::<Vec<u8>>();

        let player_start: [u8; 8] = bytemuck::cast([self.player_start.x.to_le_bytes(), self.player_start.y.to_le_bytes()]);

        let version_numer = 1_u16.to_le_bytes();

        let mut buffer = Vec::with_capacity(4 + 8 + 2 + platform_bytes.len());

        buffer.extend(FILE_HEADER);
        buffer.extend(version_numer);
        buffer.extend(player_start);
        buffer.append(&mut platform_bytes);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        file.write_all(&buffer)?;
        file.flush()?;

        Ok(())
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let mut buffer = BufReader::new(file);

        let mut header: [u8; 4] = [0; 4];
        buffer.read_exact(&mut header)?;
        assert_eq!(header, FILE_HEADER);

        let mut file_version: [u8; 2] = [0; 2];
        buffer.read_exact(&mut file_version)?;
        let file_version = u16::from_le_bytes(file_version);

        let mut player_x: [u8; 4] = [0; 4];
        buffer.read_exact(&mut player_x)?;
        let mut player_y: [u8; 4] = [0; 4];
        buffer.read_exact(&mut player_y)?;
        let player_x = f32::from_le_bytes(player_x);
        let player_y = f32::from_le_bytes(player_y);
        let player_start = vec2!(player_x, player_y);

        // start with data for one vec!
        let mut platform_data = Vec::with_capacity(20);
        buffer.read_to_end(&mut platform_data)?;

        assert_eq!(platform_data.len() % 20, 0);
        let num_of_platforms = platform_data.len() / 20;

        let platforms = (0..num_of_platforms)
            .map(|i| Platform::from_le_bytes(&platform_data[i*20..(i*20)+20]))
            .collect::<Vec<Platform>>();

        drop(buffer);

        Ok(Self {
            platforms,
            player_start,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Platform {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    pub friction: f32,
}

impl Platform {
    pub fn new(pos: Vec2<f32>, size: Vec2<f32>) -> Self {
        Self {
            pos,
            size,
            friction: 1.0,
        }
    }

    pub fn from_corners(c1: Vec2<f32>, c2: Vec2<f32>) -> Self {
        let size = c1 - c2;
        let size = vec2!(size.x.abs(), size.y.abs());
        let pos = vec2!(c1.x.min(c2.x), c1.y.min(c2.y));
        
        Self {
            size,
            pos,
            friction: 1.0,
        }
    }

    pub fn check_collision(&self, other_pos: Vec2<f32>, other_size: Vec2<f32>) -> bool {
        collision::rect_in_rect(self.pos, self.size, other_pos, other_size)
    }

    pub fn draw(&self, mat: &mut Material, renderer: &RenderInformation) {
        mat.add_rectangle(self.pos, self.size, Colour::WHITE, renderer);
    }

    pub(crate) fn to_bytes(&self) -> [u8; 20] {
        let px = self.pos.x.to_le_bytes();
        let py = self.pos.y.to_le_bytes();
        
        let sw = self.size.x.to_le_bytes();
        let sh = self.size.y.to_le_bytes();
        let f = self.friction.to_le_bytes();

        bytemuck::cast([px, py, sw, sh, f])
    }

    pub(crate) fn from_le_bytes(bytes: &[u8]) -> Self {
        let pos = vec2!(
            f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]])
        );

        let size = vec2!(
            f32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            f32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]])
        );

        let friction = f32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);

        Self {
            pos,
            size,
            friction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_level_to_file() {
        let level = InnerLevel::new(
            vec![
                Platform::new(vec2!(10.0, 200.0), vec2!(300.0, 100.0)),
                Platform::new(vec2!(0.0, 600.0), vec2!(600.0, 50.0)),
            ],
        );

        level.write_to_file("testingstuff/test_file.sgld").unwrap();
    }

    #[test]
    fn read_level_from_file() {
        let l = InnerLevel::read_from_file("testingstuff/test_file.sgld").unwrap();

        let orignial_level = InnerLevel::new(
            vec![
                Platform::new(vec2!(10.0, 200.0), vec2!(300.0, 100.0)),
                Platform::new(vec2!(0.0, 600.0), vec2!(600.0, 50.0)),
            ],
        );

        assert_eq!(l, orignial_level);
    }
}