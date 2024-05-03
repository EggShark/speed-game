use std::fmt::Debug;

use bottomless_pit::text::TextMaterial;
use bottomless_pit::vectors::Vec2;

pub mod button;

pub(crate) struct InElementText {
    pub text: TextMaterial,
    pub offset: Vec2<f32>,
}

impl Debug for InElementText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("InElementText")
            .field("text", &self.text.get_text())
            .field("offset", &self.offset)
            .finish()
    }
}