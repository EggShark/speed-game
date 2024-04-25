use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::input::MouseKey;
use bottomless_pit::material::Material;
use bottomless_pit::render::RenderInformation;
use bottomless_pit::vectors::Vec2;

use crate::collision;

pub struct Button {
    position: Vec2<f32>,
    size: Vec2<f32>,
}

impl Button {
    pub fn new(size: Vec2<f32>, pos: Vec2<f32>) -> Self {
        Self {
            position: pos,
            size,
        }
    }

    pub fn was_clicked(&self, mouse_pos: Vec2<f32>, engine: &Engine) -> bool {
        engine.is_mouse_key_pressed(MouseKey::Left) && collision::point_in_rect(mouse_pos, self.position, self.size)
    }

    pub fn render(&self, mat: &mut Material, renderer: &RenderInformation) {
        mat.add_rectangle(self.position, self.size, Colour::WHITE, renderer);
    }
}

#[derive(Debug)]
pub struct CallBackButton<T> {
    postion: Vec2<f32>,
    size: Vec2<f32>,
    callback: fn(&mut T) -> (),
}

impl<T> CallBackButton<T> {
    pub fn new(pos: Vec2<f32>, size: Vec2<f32>, callback: fn(&mut T) -> ()) -> Self {
        Self {
            postion: pos,
            size,
            callback,
        }
    }

    pub fn update(&self, mouse_pos: Vec2<f32>, engine: &Engine, args: &mut T) -> bool {
        if engine.is_mouse_key_pressed(MouseKey::Left) && collision::point_in_rect(mouse_pos, self.postion, self.size) {
            self.on_click(args);
            true
        } else {
            false
        }
    }

    pub fn render(&self, mat: &mut Material, renderer: &RenderInformation) {
        mat.add_rectangle(self.postion, self.size, Colour::WHITE, renderer);
    }

    fn on_click(&self, args: &mut T) {
        (self.callback)(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bottomless_pit::vec2;

    #[test]
    fn button_function() {
        let func = |a: &mut usize| {*a += 1};

        let b = CallBackButton::new(vec2!(0.0), vec2!(0.0), func);

        let mut num = 10;

        b.on_click(&mut num);

        assert_eq!(num, 11);
    }

    #[test]
    fn button_taking_other_function() {
        let b = CallBackButton::new(vec2!(0.0), vec2!(0.0), string_deleter);

        let mut str = String::from("hello world");
        b.on_click(&mut str);

        assert_eq!(str, String::from(""));
    }

    #[test]
    fn button_with_shared_data() {
        let func = |a: &mut usize| {*a += 1};
        let b = CallBackButton::new(vec2!(0.0), vec2!(0.0), func);

        let mut h = Holder {
            b,
            edit_data: 10,
        };

        (h.b.callback)(&mut h.edit_data);


        println!("{}", h.edit_data);
    }

    fn string_deleter(s: &mut String) {
        *s = String::new();
    }

    struct Holder {
        b: CallBackButton<usize>,
        edit_data: usize,
    }
}