


pub struct PosComponent {
    solar_pos: glam::Vec2,
    solar_system: i32,
    in_transfer: bool,
    sol_pos_history: Vec<glam::Vec2>,
}

impl PosComponent {
    pub fn new(solar_pos: glam::Vec2, solar_system: i32) -> Self {
        PosComponent {
            solar_pos,
            solar_system,
            in_transfer: false,
            sol_pos_history: Vec::new(),

        }
    }

    pub fn solar_pos(&self) -> glam::Vec2 {return self.solar_pos;}
    pub fn solar_system(&self) -> i32 {return self.solar_system;}
    pub fn in_transfer(&self) -> bool {return self.in_transfer;}

    pub fn set_solar_pos(self: &mut Self, pos: glam::Vec2) {self.solar_pos = pos;}
    pub fn set_in_transfer(self: &mut Self, b: bool) {self.in_transfer = b;}
    //Inc solar pos will add the current position to history, then increase postiion. 
    pub fn inc_solar_pos(self: &mut Self, inc: glam::Vec2) {
        self.sol_pos_history.push(self.solar_pos());
        self.solar_pos += inc;
    }
    //Inc x will just call inc_solar_pos with {inc, 0.0}
    pub fn inc_solar_pos_x(self: &mut Self, inc: f32) {
        self.inc_solar_pos(glam::Vec2::new(inc,0.0));
    }
    //Inc y will just call inc_solar_pos with {0.0, inc}
    pub fn inc_solar_pos_y(self: &mut Self, inc: f32) {
        self.inc_solar_pos(glam::Vec2::new(0.0,inc));
    }
   
    pub fn is_in_system(&self, system: i32) -> bool {
        if self.solar_system == system {return true;}
        else {return false;}
    }
    
    //FUNCTIONS
    pub fn get_orbit_final_pos(
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



