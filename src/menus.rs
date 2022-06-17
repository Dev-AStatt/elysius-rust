use crate::ecs::Entities;
use ggez::{
    graphics::{self},
    Context,
};
use glam::{f32, i32, vec2};
use crate::globs;

struct MenuBodyPositions {
    pos: glam::Vec2,
    name_pos: glam::Vec2,
    resources_pos: glam::Vec2,
    sprite_pos: glam::Vec2,
}
    impl MenuBodyPositions {
        pub fn new() -> Self {
            let pos = glam::Vec2::new(30.0,30.0);
            let name_pos = glam::Vec2::new(pos.x + 40.0, pos.y + 15.0);
            let resources_pos = glam::Vec2::new(0.0,0.0);
            let sprite_pos = glam::Vec2::new(0.0,0.0);

            MenuBodyPositions {
                pos,
                name_pos,
                resources_pos,
                sprite_pos,
            }
        }
    }

pub struct Menus {
    pub body_texture: graphics::Image,
    menu_body_pos: MenuBodyPositions,
}

impl Menus {
    pub fn new(texture: graphics::Image) -> Self {
        
        Menus {
            body_texture: texture,
            menu_body_pos: MenuBodyPositions::new(),
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
        canvas.draw(&self.body_texture, self.menu_body_pos.pos);
        //Draw Name
        let str = &ents.ent_name[ent_id];
        canvas.draw(graphics::Text::new(str)
                    .set_scale(20.0),
                    self.menu_body_pos.name_pos);

    }

}
