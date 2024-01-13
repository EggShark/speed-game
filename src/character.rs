use bottomless_pit::material::Material;
use bottomless_pit::vectors::Vec2;

struct Character {
    pos: Vec2<f32>,
    speed: Vec2<f32>,
    material: Material,
}