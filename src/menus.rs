use crate::ecs::{Entities, self};
use ggez::{
    graphics::{self},
    Context,
};



struct MenuBodyPositions {
    pos: glam::Vec2,
    name_pos: glam::Vec2,
    pos_coal_text: glam::Vec2,
    pos_radioactive_text: glam::Vec2,
}
    impl MenuBodyPositions {
        pub fn new() -> Self {
            let pos = glam::Vec2::new(30.0,30.0);
            let name_pos = glam::Vec2::new(
                pos.x + 40.0,
                pos.y + 15.0);
            let pos_coal_text = glam::Vec2::new(
                pos.x + 305.0,
                pos.y + 70.0);
            let pos_radioactive_text = glam::Vec2::new(
                pos.x + 305.0,
                pos.y + 112.0);

            MenuBodyPositions {
                pos,
                name_pos,
                pos_coal_text,
                pos_radioactive_text,
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

    menu_body_pos: MenuBodyPositions,
    
}

impl Menus {
    pub fn new(ctx: &Context) -> Self {

        Menus {
            body_texture: ecs::sprite_get(ctx, "/menu_02.png"),

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

        //Draw Name
        canvas.draw(
            graphics::Text::new(
                &ents.ent_name[ent_id]).set_scale(20.0),
                self.menu_body_pos.name_pos);
        //Draw Coal Amount
        match &ents.energy_comp[ent_id] {
            None => {},
            Some(ref e_comp) => {
                canvas.draw(
                    graphics::Text::new(
                        e_comp.fossil.to_string()).set_scale(25.0),
                        self.menu_body_pos.pos_coal_text);
                canvas.draw(
                    graphics::Text::new(
                        e_comp.radioactive.to_string()).set_scale(25.0),
                        self.menu_body_pos.pos_radioactive_text);
            },
            
        }

    }

}

