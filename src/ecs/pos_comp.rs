


pub struct PosComponent {
   solar_pos: glam::Vec2,
   solar_system: i32,
}

impl PosComponent {
    pub fn new(solar_pos: glam::Vec2, solar_system: i32) -> Self {
        PosComponent {
            solar_pos,
            solar_system,
        }
    }

    pub fn solar_pos(&self) -> glam::Vec2 {return self.solar_pos;}
    pub fn solar_system(&self) -> i32 {return self.solar_system;}

    pub fn set_solar_pos(self: &mut Self, pos: glam::Vec2) {self.solar_pos = pos;}
    pub fn inc_solar_pos(self: &mut Self, inc: glam::Vec2) {self.solar_pos += inc;}
    pub fn inc_solar_pos_x(self: &mut Self, inc: f32) {
        self.solar_pos.x += inc;
    }
    pub fn inc_solar_pos_y(self: &mut Self, inc: f32) {
        self.solar_pos.y += inc;
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



