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
        let x = 20.0;
        let y = 30.0;
        let h = 400.0;
        let w = 400.0;
        let background_color = graphics::Color::new(0.65, 0.73, 0.81, 0.5);
        //get a new meshbuilder to make our circle
        let mb = &mut graphics::MeshBuilder::new();
        //get our new circle
        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(x, y, w, h),
            background_color)
            .expect("error in creating menu background");

        mb.rectangle(
            graphics::DrawMode::stroke(3.0),
            graphics::Rect::new(x, y, w, h),
            graphics::Color::BLACK)
            .expect("error in creating menu boarder");

        mb.rectangle(
            graphics::DrawMode::stroke(3.0),
            graphics::Rect::new(x, y - 10.0, w / 3.0, 40.0),
            graphics::Color::BLACK)
            .expect("error in creating menu boarder");
            
        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(x, y - 10.0, w / 3.0, 40.0),
            background_color)
            .expect("error in creating menu boarder");
            

        let menu_outline = graphics::Mesh::from_data(ctx, mb.build());

        Menus {
            menu_outline,
        }
        
    }
}
