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
    test_sun_image: DrawingComponent,
    //ECS
    entities: Vec<Option<Entity>>,
    entities_id: Vec<EntityIndex>,
 
}


impl ElysiusMainState {
    fn new(ctx: &mut Context) -> GameResult<ElysiusMainState> {

        //Loading Sprites
        let test_sun_image = DrawingComponent {
            sprite: graphics::Image::from_path(ctx, "/Sprite-SUN_01.png", true)?,
            image_size: (128,128) };

    

        Ok(ElysiusMainState {
            test_sun_image, 
            entities: Vec::new(),
            entities_id: Vec::new(),
            })
    }

  

}

impl event::EventHandler<ggez::GameError> for ElysiusMainState {
    //Update events go in this function
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        
        

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.2, 0.2, 0.2, 1.0].into()),
        );

        //Calculate Position and Scale
        let dst = glam::Vec2::new(400.0, 400.0);
        let scale = glam::Vec2::new(1.0, 1.0);
        //Draw Sprite
        canvas.draw(&self.test_sun_image.sprite,
            graphics::DrawParam::new()
                .dest(dst)
                .scale(scale)
            );
        
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
