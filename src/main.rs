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


struct sprite_info {
    sprite: graphics::Image,
    image_size: (i32, i32),
}

//
//  To add a sprite, add a new item into MainState Struct pointing to graphics::Image
//  Then load the texture in the new() impl of MainState. Call it with the draw function. 
//

struct MainState {
    pos_x: f32,
    sun_image: graphics::Image,
    sun_image_1: sprite_info,
    
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {

        //Loading Sprites
        
        let sun_image = graphics::Image::from_path(ctx, "/Sprite-SUN_01.png", true)?;
        let sun_image_1 = sprite_info {
            sprite: graphics::Image::from_path(ctx, "/Sprite-SUN_01.png", true)?,
            image_size: (128,128) };

    

        Ok(MainState {
            sun_image,
            sun_image_1,
            pos_x: 0.0,
            })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.2, 0.2, 0.2, 1.0].into()),
        );

        //Drawing Sprite
        let dst = glam::Vec2::new(20.0, 20.0);
        let scale = glam::Vec2::new(0.5, 0.5);
        canvas.draw(&self.sun_image,
            graphics::DrawParam::new()
                .dest(dst)
                .scale(scale)
            );

        let dst = glam::Vec2::new(400.0, 400.0);
        let scale = glam::Vec2::new(1.0, 1.0);
        canvas.draw(&self.sun_image_1.sprite,
            graphics::DrawParam::new()
                .dest(dst)
                .scale(scale)
            );


        canvas.finish(ctx)?;

        Ok(())
    }
}

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
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
