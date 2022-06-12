use ggez::{
    graphics::{self},
    Context,
};
use glam::{f32, i32, vec2};
use crate::globs;

pub struct Menus {
    pub menu_outline: graphics::Mesh,
}

impl Menus {
    pub fn new(ctx: &Context) -> Self {
        //get a new meshbuilder to make our circle
        let mb = &mut graphics::MeshBuilder::new();
        //get our new circle
        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(20.0, 20.0, 400.0, 400.0),
            graphics::Color::new(0.65, 0.73, 0.81, 0.5))
            .expect("ecs new planet mesh error");

        let menu_outline = graphics::Mesh::from_data(ctx, mb.build());

        Menus {
            menu_outline,
        }
        
    }
}
