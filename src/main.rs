extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

struct SolarObject {
    solar_pos: olc::Vi2d,


}

impl SolarObject {
    fn new() -> Self {
        SolarObject {
            solar_pos: olc::Vi2d::new(20,20),
        }
    }
    fn update_body_pos(&mut self) {
        self.solar_pos.x += 1;
    }
}


struct ElysiusProgram {
    sun: SolarObject,
    tick_update: bool,
    accumulated_time: f32,
    game_tick: i32,

}

impl olc::Application for ElysiusProgram {

    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserCreate`. Your code goes here.
        
        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserUpdate`. Your code goes here.

        // Clears screen and sets black colour.
        olc::clear(olc::BLACK);
        
        self.update_current_tick(&_elapsed_time);

        if self.tick_update {
            self.sun.update_body_pos();
        }

        //prints the solar object to the screen
        olc::fill_circle(self.sun.solar_pos.x, self.sun.solar_pos.y, 4, olc::WHITE);
        
        olc::draw_string(0, 0, &self.game_tick.to_string(), olc::WHITE);
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
        Ok(())
    }

}

impl ElysiusProgram {
    //We will run the game at a fixed 60 ticks per second
    fn update_current_tick(&mut self, e_time: &f32) {
            self.tick_update = false;
            self.accumulated_time += e_time; 
            //if we have accumulated 1/60th of a second, increase the tick
            if self.accumulated_time > 0.016 {
                self.accumulated_time = 0.0;
                self.game_tick += 1;
                self.tick_update = true;
            }
    }
}


fn main() {
  let mut s_elysius = ElysiusProgram {
      sun: SolarObject::new(),
      tick_update: false,
      accumulated_time: 0.0,
      game_tick: 0,
  };
  // Launches the program in 200x100 "pixels" screen, where each "pixel" is 4x4 pixel square,
  // and starts the main game loop.
  olc::start("Elysius", &mut s_elysius, 200, 100, 4, 4).unwrap();
}