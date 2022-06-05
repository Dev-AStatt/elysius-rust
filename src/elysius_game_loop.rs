use crate::olc_pixel_game_engine as olc;


mod solar_objects;


pub struct ElysiusProgram {
    sun: solar_objects::SolarObject,
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
        olc::fill_circle(self.sun.solar_pos.0, self.sun.solar_pos.1, 4, olc::WHITE);
        
        olc::draw_string(0, 0, &self.game_tick.to_string(), olc::WHITE);
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
        Ok(())
    }

}

impl ElysiusProgram {
    pub fn new() -> Self {
        ElysiusProgram {
            sun: solar_objects::SolarObject::new(),
            tick_update: false,
            accumulated_time: 0.0,
            game_tick: 0,
        }
    }


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

