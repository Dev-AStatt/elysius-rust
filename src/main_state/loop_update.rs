
use ggez::{
    Context,
};

use super::{ms::ElysiusMainState, io};
use super::super::entities;
use super::super::ui;
use super::event_system::ElysiusEventType;
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


    pub fn update_mouse(self: &mut Self, ctx: &Context) {
        self.mouse.set_focus(io::MouseFocus::Background);

        for i in 0..self.entities_id.len() {
            if self.mouse_over_ent(i) {
                self.mouse.set_focus(io::MouseFocus::Body(i));
            }
        }   
//        if self.mouse.get_click_down() {self.mouse_down_event(ctx);}
        if self.events.check_event(ElysiusEventType::LeftMouseDown) {
            self.mouse.set_click_down(true);
            self.mouse_down_event(ctx);
        }
    }

    fn mouse_down_event(self: &mut Self, ctx: &Context) {
        match self.mouse.get_focus() {
            io::MouseFocus::Body(id) => {
                if self.entities.ent_type[id] == entities::ObjectType::Ship {
                    let p = glam::Vec2::new(self.state.screen_size().x - 400.0 ,50.0);
                    self.menus.push(
                        ui::ui_comp::UIComponent::new_ship_menu(
                            ctx, p, &self.entities, id)
                    );


                } else {
                    //add menu to menu stack
                    let p = glam::Vec2::new(50.0,50.0);
                    self.menus.push(
                        ui::ui_comp::UIComponent::new_menu_orbit_body_info(
                            &ctx,
                            p,
                            &self.entities,
                            id,
                        )
                    );
                }
            }
            io::MouseFocus::Background => {
                for i in 0..self.menus.len() {
                    if self.menus[i].menu_removeable() {
                        self.menus[i].transition_out();    
                    }
                } 
            }
            io::MouseFocus::Menu => {}
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


