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
const SCREEN_SIZE: (f32, f32) = (1024.0 ,1024.0);


mod ecs;

//
//  To add a sprite, add a new item into MainState Struct pointing to graphics::Image
//  Then load the texture in the new() impl of MainState. Call it with the draw function. 
//

//MAIN GAME STRUCT
struct ElysiusMainState {
    
    //ECS
    entities: ecs::Entities,
    entities_id: Vec<ecs::EntityIndex>,
    first_time: bool,
    game_scale: glam::Vec2,
    active_solar_system: i32,
 
}


impl ElysiusMainState {
    fn new(ctx: &mut Context) -> GameResult<ElysiusMainState> {
        //This is where you can put stuff that needs to be pre-calculated
        // //Loading Sprites
        // let test_sun_image = DrawingComponent {
        //     sprite: graphics::Image::from_path(ctx, "/Sprite-SUN_01.png", true)?,
        //     image_size: (128,128) };
        let init_ent = ecs::Entities{
            orbit_comp: Vec::new(),
            draw_comp: Vec::new(),
            solar_pos_comp: Vec::new(),
            solar_system_id: Vec::new(),
        };


        Ok(ElysiusMainState {
            entities: init_ent,
            entities_id: Vec::new(),
            first_time: true,
            game_scale: glam::Vec2::new(0.5,0.5),
            active_solar_system: 0,
            })
    }
    
    //Draw function for solar objects
    fn draw_solar_object_ecs(
        self: &Self,
        canvas: &mut graphics::Canvas,
        ent_id: usize) {
            let pos = glam::Vec2::new(self.entities.solar_pos_comp[ent_id].0,
                                      self.entities.solar_pos_comp[ent_id].1
            );
            //Draw Sprite
            canvas.draw(
                &self.entities.draw_comp[ent_id].sprite,
                graphics::DrawParam::new().dest(pos).scale(self.game_scale)
            );
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
            let screen_center = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);

            ecs::make_new_sun(
                &mut self.entities,
                &mut self.entities_id,
                sun_image,
                self.active_solar_system,                  //solar system ID
                screen_center,      //solar position
            );
            ecs::make_new_planet(
                &mut self.entities,
                &mut self.entities_id,
                planet_image,
                self.active_solar_system,                  //solar system ID
                0,                  //orbiting ent ID
                (100.0,200.0),          //solar position
                300                 //orbiting radius
            );
            //set the flag to not run this every tick.
            self.first_time = false;
        }

        ecs::inc_orbital_body_pos(&mut self.entities, self.active_solar_system);
        

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
