use crate::ecs::{Entities, sprite_get };
use ggez::{
    graphics::{self},
    Context,
};
mod color_palette;
mod orb_menu;
mod disp_item;

#[derive(PartialEq)]
pub enum MenuType {
    OrbitBodyInfo,
    ShipInfo,
    UIScreenTop,
}


pub struct UIComponent {
    menu_type: MenuType,
    pub pos: glam::Vec2,
    pub mesh: graphics::Mesh,
    display_items: Vec<disp_item::DisplayItem>,
    ent_id: usize,
}

impl UIComponent {
    pub fn new_menu_orbit_body_info(
        ctx: &Context,    
        pos: glam::Vec2,
        ent_id: usize,
    ) -> Self {
        //Get the positions of things in the menu
        let positions = orb_menu::OrbMenu::new();
        //Get disp_info into vector
        let mut disp_items: Vec<disp_item::DisplayItem> = Vec::new();
        disp_items.push(disp_item::DisplayItem::new(
            positions.display_item_pos,
            ctx,
            Some(sprite_get(ctx, "/Sprite-Coal_01.png")))
        );

        let ui = UIComponent { 
                    menu_type: MenuType::OrbitBodyInfo,
                    pos,
                    mesh: positions.get_mesh(ctx),
                    display_items: disp_items,
                    ent_id,
                }; 
        return ui;
    }

    pub fn draw_ui_comp(
        self: &Self,
        canvas: &mut graphics::Canvas,
        ents: &Entities,
    ) {
        //Draw the background
        canvas.draw(
            &self.mesh,
            graphics::DrawParam::new().dest(self.pos)
        );
        
        match self.menu_type {
            MenuType::UIScreenTop => {}
            MenuType::ShipInfo => {}
            MenuType::OrbitBodyInfo => {
                let obi_pos = orb_menu::OrbMenu::new();
                //Draw the Sprite
                canvas.draw(
                    &ents.draw_comp[self.ent_id].sprite,
                    obi_pos.spr_pos
                );
            }
        }
        //Draw each Display Item
        for i in 0..self.display_items.len() {
            self.display_items[i].draw_self(canvas, self.pos, "str".to_string());
        }
   }
    

    pub fn set_pos(&mut self, pos: glam::Vec2) {
        self.pos = pos;
    }

    pub fn menu_type(&self) -> &MenuType {
        &self.menu_type
    }

    pub fn menu_type_obi(&self) -> bool {
        if self.menu_type == MenuType::OrbitBodyInfo {
            return true;
        } else {return false;}
    }
}

