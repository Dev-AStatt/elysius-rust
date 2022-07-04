use crate::globs;


#[derive(PartialEq)]
pub enum StateType {
    Running,
    Paused,
    Menu,
}


pub struct GameState {
    state_type: StateType,
    scale: glam::Vec2,
    player_screen_offset_pos: glam::Vec2,
    active_solar_system: i32,
    first_time: bool,
    screen_size: glam::Vec2,
    screen_offset: glam::Vec2,
}

impl GameState {
    pub fn new() -> Self {
        let screen_size = glam::Vec2::new(globs::SCREEN_SIZE.0,globs::SCREEN_SIZE.1);
        let screen_offset = screen_size / 2.0;
        Self {
            state_type: StateType::Running, 
            scale: glam::Vec2::new(1.0,1.0), 
            player_screen_offset_pos: screen_offset, 
            active_solar_system: 0,
            first_time: true,
            screen_size,
            screen_offset,
        } 
    }

    //Getters and Setters
    pub fn state_type(&self) -> &StateType {&self.state_type}
    pub fn scale(&self) -> glam::Vec2 {self.scale}
    pub fn player_screen_offset_pos(&self) -> glam::Vec2 {self.player_screen_offset_pos}
    pub fn active_solar_system(&self) -> i32 {self.active_solar_system}
    pub fn first_time(&self) -> bool {self.first_time}
    pub fn screen_size(&self) -> glam::Vec2 {return self.screen_size}
    pub fn screen_offset(&self) -> glam::Vec2 {return self.screen_offset}
    pub fn set_state_type(&mut self, state_type: StateType) {
        self.state_type = state_type;
    }

    pub fn set_scale(&mut self, scale: glam::Vec2) {self.scale = scale;}
    pub fn set_player_screen_offset_pos(&mut self, screen_offset: glam::Vec2) {
        self.player_screen_offset_pos = screen_offset;
    }
    pub fn set_active_solar_system(&mut self, active_solar_system: i32) {
        self.active_solar_system = active_solar_system;
    }
    pub fn set_first_time(&mut self, first_time: bool) {
        self.first_time = first_time;
    }
    pub fn if_state_is(&self, state: StateType) -> bool {
        if state == self.state_type {return true;}
        else {return false;}
    }
}


