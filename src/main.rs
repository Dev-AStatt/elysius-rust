extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

mod elysius_game_loop;


fn main() {

    let mut s_elysius = elysius_game_loop::ElysiusProgram::new();
        
    // Launches the program in 200x100 "pixels" screen, where each "pixel" is 4x4 pixel square,
    // and starts the main game loop.
    olc::start("Elysius", &mut s_elysius, 200, 100, 4, 4).unwrap();
}