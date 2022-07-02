

use super::ui_tools::disp_item;
use super::ui_tools::orb_menu;
use super::ui_tools::transtions::{Transition, TransitionType, InOrOut, Speed};


use crate::ecs::{Entities, sprite_get };
use ggez::Context;
use ggez::graphics;

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
    transition: Transition,
}

impl UIComponent {
    pub fn new_menu_orbit_body_info(
        ctx: &Context,    
        pos: glam::Vec2,
        ents: &Entities,
        ent_id: usize,
    ) -> Self {
        //Get the positions of things in the menu
        let positions = orb_menu::OrbMenu::new();
        //Get disp_info into vector
        let mut disp_items: Vec<disp_item::DisplayItem> = Vec::new();

        //Make Name Dipsplay Item
        disp_items.push(disp_item::DisplayItem::new(
            positions.name_pos,
            disp_item::BoxSize::Large,
            ctx,
            ents.ent_name[ent_id].clone(),
            None
        ));

        //Figure out how to get this much cleaner
        match ents.energy_comp[ent_id] {
            None => {}
            Some(ref e_c) => {
                disp_items.push(disp_item::DisplayItem::new(
                    positions.display_item_pos,
                    disp_item::BoxSize::Small,
                    ctx,
                    e_c.fossil.to_string(),
                    Some(sprite_get(ctx, "/Sprite-Coal_01.png")))
                );
            }
        }
        let transition = Transition::new(
            TransitionType::Slide,
            pos - glam::Vec2::new(600.0,0.0),
            pos,
            InOrOut::IN,
            Speed::Normal,
        );

       
        let ui = UIComponent { 
                    menu_type: MenuType::OrbitBodyInfo,
                    pos: transition.get_pos(),
                    mesh: positions.get_mesh(ctx),
                    display_items: disp_items,
                    ent_id,
                    transition,
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
                    self.pos + obi_pos.spr_pos
                );
            }
        }
        //Draw each Display Item
        for i in 0..self.display_items.len() {
            self.display_items[i].draw_self(canvas, self.pos);
        }
   }
   //Fucntion will update the position of the Menu if it is in a transition state
    pub fn if_transition_update(self: &mut Self) {
        if self.transition.is_in_transition() {
            self.transition.inc_transition();
            self.pos = self.transition.get_pos();
        }
    } 


    pub fn transition_out(self: &mut Self) {
        self.transition = Transition::new(
            TransitionType::Slide,
            self.pos,
            self.pos - glam::Vec2::new(500.0,0.0), 
            InOrOut::OUT,
            Speed::Fast,
        );

    }
    //function impliments the checks to see if the menu is ready to be removed
    //checking if it is in a transition and if it has transntioned out.
    pub fn ready_to_remove(&self) -> bool {
        match self.transition.in_or_out() {
            InOrOut::IN => {return false;}
            InOrOut::OUT => {
                if !self.transition.is_in_transition() {
                    return true;
                } else { return false };
            }
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
