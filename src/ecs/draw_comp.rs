use ggez::graphics;

use crate::main_state::game_state;
use super::pos_comp::PosComponent;


pub struct DrawingComponent {
    sprite: graphics::Image,
    image_size: (i32, i32),
    sprite_offset: glam::Vec2,
    screen_pos: glam::Vec2,
    
}


impl DrawingComponent {
    pub fn new(sprite: graphics::Image, image_size: (i32, i32), 
        sprite_offset: glam::Vec2, screen_pos: glam::Vec2
    ) -> Self {
         Self { sprite, image_size, sprite_offset, screen_pos } 
    }

    //Getter Setters
    pub fn sprite(&self)        -> &graphics::Image {&self.sprite}
    pub fn image_size(&self)    -> (i32, i32)       {self.image_size}
    pub fn sprite_offset(&self) -> glam::Vec2       {self.sprite_offset}
    pub fn screen_pos(&self)    -> glam::Vec2       {self.screen_pos}

    pub fn set_screen_pos(self: &mut Self, pos: glam::Vec2) {self.screen_pos = pos;}


    pub fn update(
        self: &mut Self,
        state: &game_state::GameState,
        pos: &PosComponent,
    ) {
        self.set_screen_pos(
            pos.get_orbit_final_pos(
                state.scale(), 
                state.player_screen_offset_pos(), 
                self.sprite_offset
            )
        );
    }
}