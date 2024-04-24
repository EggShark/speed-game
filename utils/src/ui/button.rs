use bottomless_pit::vectors::Vec2;
use bottomless_pit::vec2;

struct Button<T> {
    postion: Vec2<f32>,
    size: Vec2<f32>,
    callback: fn(&mut T) -> (),
}

impl<T> Button<T> {
    pub fn new(pos: Vec2<f32>, size: Vec2<f32>, callback: fn(&mut T) -> ()) -> Self {
        Self {
            postion: pos,
            size,
            callback,
        }
    }

    pub fn on_click(&self, args: &mut T) {
        (self.callback)(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_function() {
        let func = |a: &mut usize| {*a += 1};

        let b = Button::new(vec2!(0.0), vec2!(0.0), func);

        let mut num = 10;

        b.on_click(&mut num);

        assert_eq!(num, 11);
    }

    #[test]
    fn button_taking_other_function() {
        let b = Button::new(vec2!(0.0), vec2!(0.0), string_deleter);

        let mut str = String::from("hello world");
        b.on_click(&mut str);

        assert_eq!(str, String::from(""));
    }

    #[test]
    fn button_with_shared_data() {
        let func = |a: &mut usize| {*a += 1};
        let b = Button::new(vec2!(0.0), vec2!(0.0), func);

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
        b: Button<usize>,
        edit_data: usize,
    }
}