
use ggez::Context;

use super::{ms::ElysiusMainState, io};
use super::super::entities;
use super::super::ui;
use super::event_system::{EventType, Event};
use crate::utilities;

impl ElysiusMainState {
    //Check if any menus can be deleted, then remove them
    pub fn remove_dead_menus(self: &mut Self) {
        //Pop off any menu that is ready to remove
        self.menus.retain(|i| !i.ready_to_remove());
    }

    pub fn update_menus(self: &mut Self, ctx: &Context) {
        //Draw any menus on screen
        self.menus.iter_mut().for_each(|m| {
            m.update(&self.mouse, &mut self.events);
        }); 
        self.remove_dead_menus();
        self.add_new_menus(ctx); 
    }
    fn add_new_menus(self: &mut Self, ctx: &Context) {
        //get all new menu events
        let new_events: Vec<Event> = self.events.get_events(EventType::NewMenu);
        if new_events.len() == 0 {return;}      //if no menus bail

        //for each new menu events, where menu event is i
        new_events.into_iter().for_each(|i| {
            if let Some(ent_id) = i.generated_by() {        //Get what ent_id was clicked on
                let ent_type = self.entities.ent_type[ent_id]; 
                if ent_type == entities::ObjectType::Ship {
                    self.add_menu_ship(ctx, ent_id);
                } else {
                    self.add_menu_body(ctx, ent_id);
                }
            }
        });
    } 


    fn add_menu_ship(self: &mut Self, ctx: &Context, ent_id: usize) {
        let p = glam::Vec2::new(self.state.screen_size().x - 400.0 ,50.0);
        self.menus.push(
            ui::ui_comp::UIComponent::new_ship_menu(
                ctx, p, &self.entities, ent_id)
        );
    }

    fn add_menu_body(self: &mut Self, ctx: &Context, ent_id: usize) {
        //add menu to menu stack
        let p = glam::Vec2::new(50.0,50.0);
        self.menus.push(
            ui::ui_comp::UIComponent::new_menu_orbit_body_info(
                &ctx,
                p,
                &self.entities,
                ent_id,
            )
        );

    }

    pub fn update_mouse(self: &mut Self) {
        self.mouse.set_focus(io::MouseFocus::Background);

        (0..self.entities_id.len()).for_each(|i| {
            if self.mouse_over_ent(i) {
                self.mouse.set_focus(io::MouseFocus::Body(i));
            }
        });   

        self.menus.iter().for_each(|m| {
            if m.mouse_over(self.mouse.get_pos_vec2()) {
                self.mouse.set_focus(io::MouseFocus::Menu);
            }
        });


        if self.events.check_event(EventType::LeftMouseDown) {
            self.mouse.set_click_down(true);
            self.mouse_down_event();
        }
    }

    fn mouse_down_event(self: &mut Self) {
        match self.mouse.get_focus() {
            io::MouseFocus::Body(id) => {
                self.mouse_down_entity_events(id);
            }
            io::MouseFocus::Background => {
                self.menus.iter_mut().for_each(|m| {
                    if m.menu_removeable() {
                        m.transition_out();    
                    }
                }); 
                self.events.clear_event_type(EventType::InitShipTransfer);
            }
            io::MouseFocus::Menu => {}
        }
    }
    fn mouse_down_entity_events(self: &mut Self, ent_id: usize) {
        if self.events.check_event(EventType::InitShipTransfer) {
            self.events.update_event_target(EventType::InitShipTransfer, ent_id);
        } else {
            self.events.new_event(EventType::NewMenu, Some(ent_id), None);
        }
    }

    //Function will take in an entity id and return if the mouse 
    //position is inside the entity position on the screen
    fn mouse_over_ent(&self, ent: usize) -> bool {
        let sprite_offset_scaled = self.entities.draw_comp[ent].sprite_offset() * self.state.scale();
        //This is the screen position of the entity adjusted for the scale and position on screen
        let final_pos =    self.entities.position_comp[ent].screen_pos() + sprite_offset_scaled;

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
            0,                                     
            300                                         
        );
 
        //Second Planet
        self.entities.make_new_planet(
            &mut self.entities_id,
            utilities::sprite_get(_ctx, "/Sprite-Planet_01.png"),
            self.state.active_solar_system(),                   
            0,                                     
            600                                         
        );

        //First Ship
        self.entities.make_new_ship(
            &mut self.entities_id,
            utilities::sprite_get(_ctx, "/Sprite-Ship_01.png"),
            self.state.active_solar_system(),                   
            1,                                     
            75                                         
        );

        //set the flag to not run this every tick.
        self.state.set_first_time(false);
    }
}


