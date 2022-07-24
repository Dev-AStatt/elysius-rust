use crate::main_state::game_state;

use super::{tail::Tail};


pub struct PosComponent {
    solar_pos: glam::Vec2,
    screen_pos: glam::Vec2,
    solar_system: i32,
    in_transfer: bool,
    tails: Vec<Tail>,
    history_count: i32,
}

impl PosComponent {
    pub fn new(solar_pos: glam::Vec2, solar_system: i32) -> Self {
        PosComponent {
            solar_pos,
            screen_pos: glam::Vec2::new(0.0,0.0),
            solar_system,
            in_transfer: false,
            tails: Vec::new(),
            history_count: 0,

        }
    }

    pub fn update(
        self: &mut Self,
        state: &game_state::GameState,
        sprite_offset: glam::Vec2,
        orb_center_pos: Option<glam::Vec2>,
        orbit_id: usize,
        new_pos: glam::Vec2,
    ) {
        self.set_solar_pos(new_pos);
        self.update_tails(orb_center_pos,orbit_id);        
        self.set_screen_pos(
            self.get_orbit_final_pos(
                state.scale(), 
                state.player_screen_offset_pos(), 
                sprite_offset,
            )
        );
    }

    fn update_tails(self: &mut Self, orb_center_pos: Option<glam::Vec2>, orbit_id: usize) {
        //Add to tails if we should
        if let Some(pos) = orb_center_pos {
            self.add_tail(pos, orbit_id);
        }
        //Update Tails we have 
        self.tails.iter_mut().for_each(|t| {
            t.update();
        });  
        //remove tails we dont need
        self.tails.retain(|t| !t.expired());
    }


   pub fn tails(&self) -> Vec<Tail> {
        if self.in_transfer {
            let mut alt_tails = self.tails.clone();
            alt_tails.iter_mut().for_each(|t| {
                t.set_hilight();
            });
            return alt_tails;
        } else {
            return self.tails.clone();
        }
    }

   pub fn is_in_system(&self, system: i32) -> bool {
        if self.solar_system == system {return true;}
        else {return false;}
    }
      

    fn add_tail(self: &mut Self, orb_center_pos: glam::Vec2, orbit_id: usize) {
        self.history_count += 1;
        if self.history_count > 99 {
            let new_tail = Tail::new(
                orb_center_pos - self.solar_pos,
                orbit_id,
            );
            self.tails.push(new_tail);
            self.history_count = 0;
        }
    }

    fn get_orbit_final_pos(
        &self, 
        scale: glam::Vec2, 
        player_offset: glam::Vec2,
        sprite_offset: glam::Vec2,
    ) -> glam::Vec2 {
        let sprite_pos = self.solar_pos * scale;
        let disp_adj = sprite_offset * scale;
        return sprite_pos - disp_adj + player_offset; 
    }

    pub fn solar_pos(&self) -> glam::Vec2 {return self.solar_pos;}
    pub fn solar_system(&self) -> i32 {return self.solar_system;}
    pub fn in_transfer(&self) -> bool {return self.in_transfer;}
    pub fn screen_pos(&self) -> glam::Vec2 {return self.screen_pos;}

    pub fn set_solar_pos(self: &mut Self, pos: glam::Vec2) {self.solar_pos = pos;}
    pub fn set_in_transfer(self: &mut Self, b: bool) {self.in_transfer = b;}
    //Inc solar pos will add the current position to history, then increase postiion. 
    pub fn inc_solar_pos(self: &mut Self, inc: glam::Vec2) {self.solar_pos += inc;}
    //Inc x will just call inc_solar_pos with {inc, 0.0}
    pub fn inc_solar_pos_x(self: &mut Self, inc: f32) {self.inc_solar_pos(glam::Vec2::new(inc,0.0));}
    //Inc y will just call inc_solar_pos with {0.0, inc}
    pub fn inc_solar_pos_y(self: &mut Self, inc: f32) {self.inc_solar_pos(glam::Vec2::new(0.0,inc));}
    pub fn set_screen_pos(self: &mut Self, pos: glam::Vec2) {self.screen_pos = pos }
 
}

