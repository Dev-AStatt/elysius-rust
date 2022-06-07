extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

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
    draw_info: DrawingComponents,
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

// 0------------ONE LONE CODER IMPLIMENTATION---------------0

    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        // Mirrors `olcPixelGameEngine::onUserCreate`. Your code goes here.
        

        //make our fist ECS thing
        let new_ent = Entity {
            orbit: Some(OrbitComponent {
                radius: 5,
                 angle: 0 }),
            draw_info: DrawingComponents {
                circle_size: 5, 
                color: (255,255,0)},
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

// 0------------ELYSIUS FUNCTIONS---------------0
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
 
    //Clunky way of trying to figure out drawing with this model
    fn draw_solar_object_ecs(self: &Self, ent_id: usize) {
        //Check if entities at given id is not None
        match self.entities[ent_id]  {
            None => return,
            Some(ref ent) => {      //if valid, set ent to a reference to self.entities[ent_id]
                //Set col to be an OLC pixel color of 
                let col = olc::Pixel::rgb(ent.draw_info.color.0, ent.draw_info.color.1,ent.draw_info.color.2);
                olc::fill_circle(ent.solar_pos.0, ent.solar_pos.1, ent.draw_info.circle_size, col)
                    
            }
        }
    }  
}


//  0---------------------------------------------------------------------------------0
//  | MAIN FUNCTION                                                                   |
//  0---------------------------------------------------------------------------------0

fn main() {

    let mut s_elysius = ElysiusProgram::new();
        
    // Launches the program in 200x100 "pixels" screen, where each "pixel" is 4x4 pixel square,
    // and starts the main game loop.
    olc::start("Elysius", &mut s_elysius, 200, 100, 4, 4).unwrap();
}