
use ggez::{
    Context,
};

use super::{ms::ElysiusMainState, io};
use crate::utilities;

impl ElysiusMainState {
    //Check if any menus can be deleted, then remove them
    pub fn remove_dead_menus(self: &mut Self) {
        //Pop off any menu that is ready to remove
        self.menus.retain(|i| !i.ready_to_remove());
    }

    pub fn update_menus(self: &mut Self) {
        //Draw any menus on screen
        for i in 0..self.menus.len() {
            self.menus[i].update(self.mouse.get_pos_vec2(), &self.events);
        } 
        self.remove_dead_menus();
   }


    pub fn update_mouse(self: &mut Self) {
        self.mouse.set_focus(io::MouseFocus::Background);

        for i in 0..self.entities_id.len() {
            let sprite_offset_scaled = self.entities.draw_comp[i].sprite_offset() * self.state.scale();
            let adj_pos_for_input = (
                self.entities.draw_comp[i].screen_pos().x + sprite_offset_scaled.x, 
                self.entities.draw_comp[i].screen_pos().y + sprite_offset_scaled.y
            );

            if utilities::point_in_circle(&self.mouse.get_pos_f32(),
                adj_pos_for_input, 
            self.entities.draw_comp[i].sprite_offset().x * self.state.scale().x,
            ) {
                self.mouse.set_focus(io::MouseFocus::Body(i));
            }
        }    
    }

    fn mouse_over_ent(&self, ent: usize) -> bool {
        let sprite_offset_scaled = self.entities.draw_comp[ent].sprite_offset() * self.state.scale();
        let adj_pos_for_input = (
            self.entities.draw_comp[ent].screen_pos().x + sprite_offset_scaled.x, 
            self.entities.draw_comp[ent].screen_pos().y + sprite_offset_scaled.y
        );

        if utilities::point_in_circle(&self.mouse.get_pos_f32(),
            adj_pos_for_input, 
        self.entities.draw_comp[ent].sprite_offset().x * self.state.scale().x,
        ) {return true;}
        else {return false;}
    }


    pub fn gen_new_system(self: &mut Self, _ctx: &mut Context) {

        //First Sun
        self.entities.make_new_sun(
            &mut self.entities_id,
            utilities::sprite_get(_ctx, "/Sprite-SUN_02.png"),
            self.state.active_solar_system()                       
        );            
            
        //First Planet
        self.entities.make_new_planet(
            &mut self.entities_id,
            utilities::sprite_get(_ctx, "/Sprite-Planet_01.png"),
            self.state.active_solar_system(),                   
            &_ctx,
            0,                                     
            300                                         
        );

        //First Ship
        self.entities.make_new_ship(
            &mut self.entities_id,
            utilities::sprite_get(_ctx, "/Sprite-Ship_01.png"),
            self.state.active_solar_system(),                   
            &_ctx,
            1,                                     
            75                                         
        );

        //set the flag to not run this every tick.
        self.state.set_first_time(false);
    }
}


