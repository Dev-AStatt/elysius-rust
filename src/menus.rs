use ggez::{
    graphics::{self},
    Context,
};
use glam::{f32, i32, vec2};
use crate::globs;

pub struct Menus {
    pub body_texture: graphics::Image,
}

impl Menus {
    pub fn new(texture: graphics::Image) -> Self {
        Menus {
            body_texture: texture,
        }
    }
}
