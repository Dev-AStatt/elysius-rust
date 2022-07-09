
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
            if self.mouse_over_ent(i) {
                self.mouse.set_focus(io::MouseFocus::Body(i));
            }
        }    
    }

    //Function will take in an entity id and return if the mouse 
    //position is inside the entity position on the screen
    fn mouse_over_ent(&self, ent: usize) -> bool {
        let sprite_offset_scaled = self.entities.draw_comp[ent].sprite_offset() * self.state.scale();
        //This is the screen position of the entity adjusted for the scale and position on screen
        let final_pos =    self.entities.draw_comp[ent].screen_pos() + sprite_offset_scaled;

        let new_rad = self.entities.draw_comp[ent].sprite_offset().x * self.state.scale().x;
        if utilities::point_in_circle_vec2(
            &self.mouse.get_pos_vec2(),
            final_pos,
            new_rad
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


