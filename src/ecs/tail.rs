use crate::main_state::game_state;

#[derive(Copy, Clone)]
pub struct Tail {
    pos: glam::Vec2,
    orbit_id: usize,
    lifetime: i32,
    hilighted: bool,
    expired: bool,
}

impl Tail {
    pub fn new(pos: glam::Vec2, orbit_id: usize) -> Self {
        Tail {
            pos,
            orbit_id,
            lifetime: 4000,
            hilighted: false,
            expired: false,
        }
    }    
    pub fn update(self: &mut Self) {
        self.lifetime -= 1;
        if self.lifetime <= 0 {self.expired = true;}
    }

    pub fn calc_final_tail_pos(
        &self, 
        state: &game_state::GameState,
        orbit_pos: glam::Vec2,
    ) -> glam::Vec2 {
            let scaled_pos = (orbit_pos - self.pos) * state.scale();
            return scaled_pos + state.player_screen_offset_pos();
        }

    pub fn pos(&self) -> glam::Vec2 {return self.pos}
    pub fn orbit_id(&self) -> usize {return self.orbit_id}
    pub fn lifetime(&self) -> i32 {return self.lifetime}
    pub fn hilighted(&self) -> bool {return self.hilighted}
    pub fn expired(&self) -> bool {return self.expired}
    pub fn set_pos(self: &mut Self, value: glam::Vec2) {self.pos = value}
    pub fn set_hilight(self: &mut Self) {self.hilighted = true;}
}





