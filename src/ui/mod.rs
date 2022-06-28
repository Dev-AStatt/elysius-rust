use crate::ecs::Entities;
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
        pos_init: (f32,f32),
        ent_id: usize,
    ) -> Self {
        let menu_type = MenuType::OrbitBodyInfo;
        //Get the positions of things in the menu
        let positions = orb_menu::OrbMenuPos::new();
        //make position out of pos_init
        let pos = glam::Vec2::new(pos_init.0, pos_init.1);
        
        //make a new mesh for us to add things to
        let mb = &mut graphics::MeshBuilder::new();
        orb_menu::add_bkgr_orb_bod_to_mesh(mb, &positions);
        //build mesh
        let mesh =  graphics::Mesh::from_data(ctx, mb.build());

        //Get buttons into vector
        let mut disp_items: Vec<disp_item::DisplayItem> = Vec::new();
        disp_items.push(disp_item::DisplayItem::new(positions.display_item_pos, ctx, None));

        let ui = UIComponent { 
                    menu_type,
                    pos,
                    mesh,
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
                let obi_pos = orb_menu::OrbMenuPos::new();
                //Draw the Sprite
                canvas.draw(
                    &ents.draw_comp[self.ent_id].sprite,
                    obi_pos.spr_pos
                );
            }
        }
        //Draw each Display Item
        for i in 0..self.display_items.len() {
            self.display_items[i].draw_self(canvas, self.pos);
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

