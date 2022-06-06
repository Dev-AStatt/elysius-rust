use crate::olc_pixel_game_engine as olc;


mod solar_objects;

pub struct ElysiusProgram {
    system_1: solar_objects::SolarSystem,
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
            //Use this to update the solar system positions 
            self.system_1.update_bodies();
        }

        //Draw Solar System
        self.draw_solar_object_by_ref(&self.system_1.sun);
        for n in 0..self.system_1.numb_of_bodies() {
            self.draw_solar_object_by_ref(&self.system_1.orbiting_bodies[n as usize])
        }
        
        

        //self.draw_solar_object_by_ref(&self.system_1);
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
            system_1: solar_objects::SolarSystem::new((5,5)),
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


//  0---------------------------0
//  | Drawing Functions         |
//  0---------------------------0
    //function takes in the solar object to draw and then prints it into olc
    fn draw_solar_object_by_ref(self: &Self, obj: &solar_objects::SolarObject) {
        //create a olc::Pixel from the rgb touple stored in solar object
        let col = olc::Pixel::rgb(obj.color.0, obj.color.1, obj.color.2);  
        olc::fill_circle(obj.solar_pos.0, obj.solar_pos.1, obj.size, col);
    }
}

