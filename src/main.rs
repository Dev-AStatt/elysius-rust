//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{
    event,
    graphics::{self, Color},
    Context, GameResult,
};
use glam::*;
use std::{env, path};

//GLOBAL VALUE for screen size
const SCREEN_SIZE: (f32, f32) = (1024 as f32,1024 as f32);



// 0------------------Start of ECS Sstem---------------------------------------0
type EntityIndex = usize;

struct OrbitalComponent {
    orbiting_ent_id: usize,
    radius: i32,
    angle: f32,
}

struct DrawingComponent {
    sprite: graphics::Image,
    image_size: (i32, i32),
    
}

struct Entity {
    orbit: Option<OrbitalComponent>,
    draw_info: DrawingComponent,
    solar_pos: (i32, i32),
    solar_system_id: i32,
}


// 0--------------------End of ECS Sstem---------------------------------------0


//
//  To add a sprite, add a new item into MainState Struct pointing to graphics::Image
//  Then load the texture in the new() impl of MainState. Call it with the draw function. 
//

//MAIN GAME STRUCT
struct ElysiusMainState {
    
    //ECS
    entities: Vec<Option<Entity>>,
    entities_id: Vec<EntityIndex>,
    first_time: bool,
 
}


impl ElysiusMainState {
    fn new(ctx: &mut Context) -> GameResult<ElysiusMainState> {
        //This is where you can put stuff that needs to be pre-calculated
        // //Loading Sprites
        // let test_sun_image = DrawingComponent {
        //     sprite: graphics::Image::from_path(ctx, "/Sprite-SUN_01.png", true)?,
        //     image_size: (128,128) };
        Ok(ElysiusMainState {
            entities: Vec::new(),
            entities_id: Vec::new(),
            first_time: true,
            })
    }
    
    fn make_new_sun(&mut self, n_sol_sys_id: i32, n_sol_pos: (i32 ,i32), 
    n_sprite: graphics::Image) {
    
        let new_ent = Entity {
            orbit: None,
            draw_info: DrawingComponent {
                sprite: n_sprite,
                image_size: (128,128)  },
            solar_pos: n_sol_pos,
            solar_system_id: n_sol_sys_id,
        };
        self.entities.push(Some(new_ent));
        self.entities_id.push(self.entities_id.len());
    }

    //pass in all the setup to add a new orbital body into the data structure
    //IF GIVEN NO ORBITAL RADIOUS IT IS SET AS NONE
    fn make_new_planet(&mut self, n_sol_sys_id: i32, n_orbiting_id: usize,
        n_sol_pos: (i32,i32), n_orb_rad: i32, n_sprite: graphics::Image) {
       //make a new entity
       let new_ent = Entity {
            orbit: Some(OrbitalComponent {
                orbiting_ent_id: n_orbiting_id,
                radius: n_orb_rad,
                angle: 0.0             }),
            draw_info: DrawingComponent {
                sprite: n_sprite,
                image_size: (128,128)  },
            solar_pos: n_sol_pos,
            solar_system_id: n_sol_sys_id,
        };
        //push into entities vector
        self.entities.push(Some(new_ent));
        self.entities_id.push(self.entities_id.len());
    }

    fn draw_solar_object_ecs(
        self: &Self,
        canvas: &mut graphics::Canvas,
        ent_id: usize) {
        //Check if entities at given id is not None
        match self.entities[ent_id]  {
            None => return,
            Some(ref ent) => {      //if valid, set ent to a reference to self.entities[ent_id]
              //Calculate Position and Scale
                let dst = glam::Vec2::new(ent.solar_pos.0 as f32, ent.solar_pos.1 as f32);
                let scale = glam::Vec2::new(1.0, 1.0);
                //Draw Sprite
                canvas.draw(&ent.draw_info.sprite,
                    graphics::DrawParam::new()
                        .dest(dst)
                        .scale(scale)
                    ); 
              
                    
            }
        }
    }

}

impl event::EventHandler<ggez::GameError> for ElysiusMainState {
    //Update events go in this function
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //Create Inital test scene
        if self.first_time {
            //Load Textures
            let sun_image = graphics::Image::from_path(_ctx, "/Sprite-SUN_01.png", true)?;
            let planet_image = graphics::Image::from_path(_ctx, "/Sprite-Planet_01.png", true)?;
            //Calc the center of the screen
            let screen_center = (SCREEN_SIZE.0 as i32 / 2, SCREEN_SIZE.1 as i32 / 2);

            self.make_new_sun(0, screen_center, sun_image);
            self.make_new_planet(0, 0, (100,25), 100, planet_image);
            //set the flag to not run this every tick.
            self.first_time = false;
        }
        

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.2, 0.2, 0.2, 1.0].into()),
        );

         //Draw ECS Ent
         for i in 0..self.entities_id.len() {
            self.draw_solar_object_ecs(&mut canvas, i);
        }
        
        //Concatinating strings is dumb
        let mut str = String::from("Tick: ");
        str.push_str(&ctx.time.ticks().to_string());
        //Draw the current tick to the screen
        canvas.draw(graphics::Text::new(str)
                    .set_scale(10.0),
                    glam::Vec2::new(0.0,0.0));


        //Draw the FPS counter
        ctx.gfx.set_window_title(&format!(
            "Elysius - {:.0} FPS", ctx.time.fps()));
        

        //Nothing after this, pushes all the draws to the graphics card
        canvas.finish(ctx)?;
        Ok(())
    }

    
}






// 0---------------------PROGRAM MAIN------------------------------------------0

pub fn main() -> GameResult {
    //added in to add resources dir
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    //End of Resource Directory Stuff
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        //Tell context builder where to find the resources for our game
        .add_resource_path(resource_dir)
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("super_simple"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));

    // And finally we attempt to build the context and create the window. If it fails, we panic with the message
    // "Failed to build ggez context"
    let (mut ctx, event_loop) = cb.build()?;
    let state = ElysiusMainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
