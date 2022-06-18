use crate::ecs::{Entities, self};
use ggez::{
    graphics::{self},
    Context,
};



struct MenuBodyPositions {
    pos: glam::Vec2,
    name_pos: glam::Vec2,
    pos_coal_sprite: glam::Vec2,
    pos_energy_sprite: glam::Vec2,
}
    impl MenuBodyPositions {
        pub fn new() -> Self {
            let pos = glam::Vec2::new(30.0,30.0);
            let name_pos = glam::Vec2::new(
                pos.x + 40.0,
                pos.y + 15.0);
            let pos_coal_sprite = glam::Vec2::new(
                pos.x + 265.0,
                pos.y + 64.0);
            let pos_energy_sprite = glam::Vec2::new(
                pos.x + 265.0,
                pos.y + 140.0);

            MenuBodyPositions {
                pos,
                name_pos,
                pos_coal_sprite,
                pos_energy_sprite,
            }
        }
    }

impl Default for MenuBodyPositions {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Menus {
    pub body_texture: graphics::Image,
    coal_texture: graphics::Image,
    energy_texture: graphics::Image,
    menu_body_pos: MenuBodyPositions,
    
}

impl Menus {
    pub fn new(ctx: &Context) -> Self {

        Menus {
            body_texture: ecs::sprite_get(ctx, "/menu_01.png"),
            coal_texture: ecs::sprite_get(ctx, "/Sprite-Coal_01.png"),
            energy_texture: ecs::sprite_get(ctx, "/Sprite-Energy_01.png"),
            menu_body_pos: MenuBodyPositions::new(),
        }
    }

    pub fn draw_body_info_menu(
        self: &Self,
        canvas: &mut graphics::Canvas,
        ents: &Entities,
        ent_id: usize,
    ) {
        //Draw Menu
        canvas.draw(
            &self.body_texture,
            self.menu_body_pos.pos);
        //Draw Sprites
        canvas.draw(
            &self.coal_texture,
            self.menu_body_pos.pos_coal_sprite);
        canvas.draw(
            &self.energy_texture,
            self.menu_body_pos.pos_energy_sprite);
        //Draw Name
        canvas.draw(
            graphics::Text::new(&ents.ent_name[ent_id])
                    .set_scale(20.0),
            self.menu_body_pos.name_pos);
        //Draw Icons

    }

}


