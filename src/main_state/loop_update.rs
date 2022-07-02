
use ggez::{
    Context,
};

use super::ms::ElysiusMainState;

use super::super::ecs;





impl ElysiusMainState {
    //Check if any menus can be deleted, then remove them
    pub fn remove_dead_menus(self: &mut Self) {
        //Pop off any menu that is ready to remove
        self.menus.retain(|i| !i.ready_to_remove());
    }

    pub fn update_menus(self: &mut Self) {
        //Draw any menus on screen
        for i in 0..self.menus.len() {
            self.menus[i].if_transition_update();
        } 
        self.remove_dead_menus();
    }


    pub fn gen_new_system(self: &mut Self, _ctx: &mut Context) {

        //First Sun
        self.entities.make_new_sun(
            &mut self.entities_id,
            ecs::sprite_get(_ctx, "/Sprite-SUN_02.png"),
            self.active_solar_system                       
        );            
            
        //First Planet
        self.entities.make_new_planet(
            &mut self.entities_id,
            ecs::sprite_get(_ctx, "/Sprite-Planet_01.png"),
            self.active_solar_system,                   
            &_ctx,
            0,                                     
            300                                         
        );

        //First Ship
        self.entities.make_new_ship(
            &mut self.entities_id,
            ecs::sprite_get(_ctx, "/Sprite-Ship_01.png"),
            self.active_solar_system,                   
            &_ctx,
            1,                                     
            75                                         
        );

        //set the flag to not run this every tick.
        self.first_time = false;
    }
}


