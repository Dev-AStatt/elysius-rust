use crate::olc_pixel_game_engine as olc;


//mod solar_objects;


// 0------------Start of ECS System---------------0
type EntityIndex = usize;


struct OrbitComponent {
    radius: i32,
    angle: i32,
}
struct DrawingComponents {
    circle_size: i32,
    color: (u8,u8,u8),
}


struct Entity {
    orbit: Option<OrbitComponent>,
    draw_info: Option<DrawingComponents>,
    solar_pos: (i32,i32),
}

// 0------------End of ECS System---------------0


pub struct ElysiusProgram {
    //Old stuff for OOP Solar System
    //system_1: solar_objects::SolarSystem,

    //ECS
    entities: Vec<Option<Entity>>,
    entities_ID: Vec<EntityIndex>,


    tick_update: bool,
    accumulated_time: f32,
    game_tick: i32,

}

impl olc::Application for ElysiusProgram {

    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserCreate`. Your code goes here.
        

        //make our fist ECS thing
        let new_ent = Entity {
            orbit: Some(OrbitComponent {
                radius: 5,
                 angle: 0 }),
            draw_info: Some(DrawingComponents {
                circle_size: 5, 
                color: (255,255,0)}),
            solar_pos: (100,50),
        };
        self.entities.push(Some(new_ent));
        self.entities_ID.push(0);

        //end of ecs thing


        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserUpdate`. Your code goes here.

        // Clears screen and sets black colour.
        olc::clear(olc::BLACK);
        
        self.update_current_tick(&_elapsed_time);

        if self.tick_update {
            //Use this to update the solar system positions 
            //self.system_1.update_bodies();
        }

        //Draw Solar System
        // self.draw_solar_object_by_ref(&self.system_1.sun);
        // for n in 0..self.system_1.numb_of_bodies() {
        //     self.draw_solar_object_by_ref(&self.system_1.orbiting_bodies[n as usize])
        // }

        //Draw ECS Ent
        self.draw_solar_object_ecs(0);

        
        

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
            //system_1: solar_objects::SolarSystem::new((5,5)),
            tick_update: false,
            accumulated_time: 0.0,
            game_tick: 0,
            entities: Vec::new(),
            entities_ID: Vec::new(),
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
    fn draw_solar_object_ecs(self: &Self, ent_id: usize) {
        match self.entities[ent_id]  {
            None => return,
            Some(ref ent) => {
                match ent.draw_info {
                    None => return,
                    Some(ref draw_ent) => {
                        let col = olc::Pixel::rgb(draw_ent.color.0, draw_ent.color.1,draw_ent.color.2);
                        olc::fill_circle(ent.solar_pos.0, ent.solar_pos.1, draw_ent.circle_size, col)
                    }
                }
            }
        }  
    }
        //let col = olc::Pixel::rgb(self.entities[ent_id].);  
}



    //function takes in the solar object to draw and then prints it into olc
    // fn draw_solar_object_by_ref(self: &Self, obj: &solar_objects::SolarObject) {
    //     //create a olc::Pixel from the rgb touple stored in solar object
    //     let col = olc::Pixel::rgb(obj.color.0, obj.color.1, obj.color.2);  
    //     olc::fill_circle(obj.solar_pos.0, obj.solar_pos.1, obj.size, col);
    // }


