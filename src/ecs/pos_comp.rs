use crate::main_state::game_state;




pub struct PosComponent {
    solar_pos: glam::Vec2,
    screen_pos: glam::Vec2,
    solar_system: i32,
    in_transfer: bool,
    screen_pos_history: Vec<glam::Vec2>,
    history_count: i32,
}

impl PosComponent {
    pub fn new(solar_pos: glam::Vec2, solar_system: i32) -> Self {
        PosComponent {
            solar_pos,
            screen_pos: glam::Vec2::new(0.0,0.0),
            solar_system,
            in_transfer: false,
            screen_pos_history: Vec::new(),
            history_count: 0,

        }
    }

    pub fn solar_pos(&self) -> glam::Vec2 {return self.solar_pos;}
    pub fn solar_system(&self) -> i32 {return self.solar_system;}
    pub fn in_transfer(&self) -> bool {return self.in_transfer;}
    pub fn sol_pos_history(&self) -> Vec<glam::Vec2> {return self.screen_pos_history.clone();}
    pub fn screen_pos(&self) -> glam::Vec2 {return self.screen_pos;}

    pub fn set_solar_pos(self: &mut Self, pos: glam::Vec2) {self.solar_pos = pos;}
    pub fn set_in_transfer(self: &mut Self, b: bool) {self.in_transfer = b;}
    //Inc solar pos will add the current position to history, then increase postiion. 
    pub fn inc_solar_pos(self: &mut Self, inc: glam::Vec2) {
        self.solar_pos += inc;
    }
    //Inc x will just call inc_solar_pos with {inc, 0.0}
    pub fn inc_solar_pos_x(self: &mut Self, inc: f32) {self.inc_solar_pos(glam::Vec2::new(inc,0.0));}
    //Inc y will just call inc_solar_pos with {0.0, inc}
    pub fn inc_solar_pos_y(self: &mut Self, inc: f32) {self.inc_solar_pos(glam::Vec2::new(0.0,inc));}
    pub fn set_screen_pos(self: &mut Self, pos: glam::Vec2) {
        self.add_to_history();
        self.screen_pos = pos;
    }

    pub fn is_in_system(&self, system: i32) -> bool {
        if self.solar_system == system {return true;}
        else {return false;}
    }
   
    pub fn update(
        self: &mut Self,
        state: &game_state::GameState,
        sprite_offset: glam::Vec2,
    ) {
       self.set_screen_pos(
            self.get_orbit_final_pos(
                state.scale(), 
                state.player_screen_offset_pos(), 
                sprite_offset,
            )
        ) 
    }
    fn add_to_history(self: &mut Self) {
        self.history_count += 1;
        if self.history_count > 99 {
            self.screen_pos_history.push(self.screen_pos);
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
}



