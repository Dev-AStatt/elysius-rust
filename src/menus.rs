use crate::ecs::Entities;
use ggez::{
    graphics::{self},
    Context,
};
use glam::{f32, i32, vec2};
use crate::globs;

pub struct Menus {
    pub body_texture: graphics::Image,
    menu_pos: glam::Vec2,
    menu_name_pos: glam::Vec2,
}

impl Menus {
    pub fn new(texture: graphics::Image) -> Self {
        let menu_pos = glam::Vec2::new(30.0,30.0);
        let menu_name_pos = glam::Vec2::new(menu_pos.x + 40.0, menu_pos.y + 15.0);

        Menus {
            body_texture: texture,
            menu_pos,
            menu_name_pos, 
        }
    }

    pub fn draw_body_info_menu(
        self: &Self,
        ctx: &mut ggez::Context,
        canvas: &mut graphics::Canvas,
        ents: &Entities,
        ent_id: usize,
    ) {
        //Draw Menu
        canvas.draw(&self.body_texture, self.menu_pos);
        //Draw Name
        let str = &ents.ent_name[ent_id];
        canvas.draw(graphics::Text::new(str)
                    .set_scale(20.0),
                    self.menu_name_pos);

    }

}
