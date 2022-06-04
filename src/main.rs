extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

struct Sun {
    pos: olc::Vi2d,


}

impl Sun {
    fn new() -> Self {
        Self {
            pos: olc::Vi2d::new(20,20),
        }
    }
}


struct ElysiusProgram {
    sun: Sun,

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
    // Prints the string starting at the position (40, 40) and using white colour.
    //olc::draw_string(40, 40, "Hello, World!", olc::WHITE)?;
    olc::fill_circle(self.sun.pos.x, self.sun.pos.y, 4, olc::WHITE);
    
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
    Ok(())
  }
}

fn main() {
  let mut s_elysius = ElysiusProgram {
      sun: Sun::new(),
  };
  // Launches the program in 200x100 "pixels" screen, where each "pixel" is 4x4 pixel square,
  // and starts the main game loop.
  olc::start("Elysius", &mut s_elysius, 200, 100, 4, 4).unwrap();
}