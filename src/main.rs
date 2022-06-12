#![allow(clippy::unnecessary_wraps)]

use ggez::{
    event::{self, MouseButton},
    graphics::{self},
    Context, GameResult,
    input::keyboard::{KeyCode, KeyInput},
};
use glam::*;
use std::{env, path};

//GLOBAL VALUE for screen size
const SCREEN_SIZE: (f32, f32) = (1024.0 ,1024.0);
const SCREEN_OFFSET: (f32, f32) = (512.0, 512.0);

#[derive(PartialEq)]
enum GameState {
    running,
    paused,
    menu,
}

#[derive(PartialEq)]
enum MouseFocus {
    background,
    body(usize),
    menu,
}


mod ecs;

//MAIN GAME STRUCT
struct ElysiusMainState {
    //ECS
    entities: ecs::Entities,
    entities_id: Vec<ecs::EntityIndex>,
    first_time: bool,
    game_scale: glam::Vec2,
    active_solar_system: i32,
    current_game_state: GameState,
    //mouse stuff
    current_mouse_focus: MouseFocus,
    current_mouse_pos: (f32, f32),
    mouse_click_pos: (f32, f32),
    mouse_click_down: bool,
}

impl ElysiusMainState {
    fn new(_ctx: &mut Context) -> GameResult<ElysiusMainState> {
        //This is where you can put stuff that needs to be pre-calculated

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
            game_scale: glam::Vec2::new(1.0,1.0),
            active_solar_system: 0,
            current_game_state: GameState::running,
            current_mouse_focus: MouseFocus::background,
            current_mouse_pos: (0.0, 0.0),
            mouse_click_pos: (0.0, 0.0),
            mouse_click_down: false,
            })
    }
    
    //Draw function for solar objects and their rings
    fn draw_solar_object_ecs(
        self: &Self,
        canvas: &mut graphics::Canvas,
        ent_id: usize
    ) {

        //Orbit Circle Component
        match &self.entities.orbit_comp[ent_id] {
            None => {}
            Some(ref orb) => { 
                //get the final position of the circle
                let circle_pos = glam::Vec2::new(
                    (self.entities.solar_pos_comp[orb.orbiting_ent_id].0 * self.game_scale.x) + SCREEN_OFFSET.0,
                    (self.entities.solar_pos_comp[orb.orbiting_ent_id].1 * self.game_scale.y) + SCREEN_OFFSET.1 
                );
                //Draw the circle
                canvas.draw(&orb.orbit_circle, 
                    graphics::DrawParam::new()
                        .scale(self.game_scale)
                        .dest(circle_pos)
                );    
            }
        }
        //Draw the sprite
        canvas.draw(&self.entities.draw_comp[ent_id].sprite,
            graphics::DrawParam::new()
                .dest(self.entities.draw_comp[ent_id].screen_pos)
                .scale(self.game_scale)
        );
    }

    fn get_orbit_final_pos(self: &Self, ent_id: usize) -> glam::Vec2 {
        let sprite_pos = glam::Vec2::new(
            self.entities.solar_pos_comp[ent_id].0 * self.game_scale.x,
            self.entities.solar_pos_comp[ent_id].1 * self.game_scale.y
        );
        let disp_adj = glam::Vec2::new(
            SCREEN_OFFSET.0 - (self.entities.draw_comp[ent_id].sprite_offset.0 * self.game_scale.x),
            SCREEN_OFFSET.1 - (self.entities.draw_comp[ent_id].sprite_offset.1 * self.game_scale.y),
        );
        return sprite_pos + disp_adj;
    }

    fn mouse_in_circle(self: &Self, ent_id: usize) -> bool {
        
        return false;
    }


}

impl event::EventHandler<ggez::GameError> for ElysiusMainState {
    //Update events go in this function
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //Create Inital test scene
        if self.first_time {
            //Load Textures
            let sun_image = graphics::Image::from_path(_ctx, "/Sprite-SUN_02.png", true)?;
            let planet_image = graphics::Image::from_path(_ctx, "/Sprite-Planet_01.png", true)?;
            let moon_image = graphics::Image::from_path(_ctx, "/Sprite-Moon_01.png", true)?;
            //Calc the center of the screen
            
            //First Sun
            ecs::make_new_sun(
                &mut self.entities,
                &mut self.entities_id,
                sun_image,
                self.active_solar_system,                   //solar system ID
                (0.0,0.0),                                  //solar position
            );
            //First Planet
            ecs::make_new_orbiting_body(
                &mut self.entities,
                &mut self.entities_id,
                &_ctx,
                planet_image,
                self.active_solar_system,                   //solar system ID
                0,                                          //orbiting ent ID
                300                                         //orbiting radius
            );
            //First Planet
            ecs::make_new_orbiting_body(
                &mut self.entities,
                &mut self.entities_id,
                &_ctx,
                moon_image,
                self.active_solar_system,                   //solar system ID
                1,                                          //orbiting ent ID
                100                                         //orbiting radius
            );
            //set the flag to not run this every tick.
            self.first_time = false;
        }
        //0----------------------GAME UPDATES----------------------------------0
        //Reset the mouse focus
        self.current_mouse_focus = MouseFocus::background;

        for i in 0..self.entities_id.len() {
            //For all entities that are on screen
            if self.entities.solar_system_id[i] == self.active_solar_system {
                //update the final positions of entites
                self.entities.draw_comp[i].screen_pos = self.get_orbit_final_pos(i);
                //update mouse focus
                let offset = (self.entities.draw_comp[i].sprite_offset.0 as f32 * self.game_scale.x,
                              self.entities.draw_comp[i].sprite_offset.1 as f32 * self.game_scale.y); 
                let for_mouse_pos = (
                    self.entities.draw_comp[i].screen_pos.x + offset.0, 
                    self.entities.draw_comp[i].screen_pos.y + offset.1
                );
                if ecs::point_in_object(&self.current_mouse_pos,
                    for_mouse_pos, 
                self.entities.draw_comp[i].sprite_offset.0 as f32 * self.game_scale.x,
                ) {
                    // print!("mouse: `{{` {}, {} `}}` ",
                    //     self.current_mouse_pos.0,
                    //     self.current_mouse_pos.1
                    // );
                    // print!("Withn point: `{{` {}, {} `}}`", for_mouse_pos.0,for_mouse_pos.1 );
                    // println!("With Radius: {}", self.entities.draw_comp[i].sprite_offset.0 as f32 * self.game_scale.x);
                    self.current_mouse_focus = MouseFocus::body(i);
                }
            }
        }

        //GameState Running
        if self.current_game_state == GameState::running {
            ecs::inc_orbital_body_pos(&mut self.entities, self.active_solar_system);
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
             //add an if in active system
            self.draw_solar_object_ecs(&mut canvas, i);
        }

        //Concatinating strings is dumb
        let mut str = String::from("Tick: ");
        str.push_str(&ctx.time.ticks().to_string());
        //Draw the current tick to the screen
        canvas.draw(graphics::Text::new(str)
                    .set_scale(10.0),
                    glam::Vec2::new(0.0,0.0));

        //Draw the focus mode
        let mut focus_str = String::from("Mouse Focus: ");
        match self.current_mouse_focus {
            MouseFocus::background => {
                focus_str.push_str("Background");
            }
            MouseFocus::body(id) => {
                focus_str.push_str(&("Entity ".to_owned()+ &id.to_string()));
            }
            MouseFocus::menu => {
                focus_str.push_str("Menu");
            }
        }
        canvas.draw(graphics::Text::new(focus_str)
                    .set_scale(10.0),
                    glam::Vec2::new(0.0,10.0));

        //Draw the FPS counter
        ctx.gfx.set_window_title(&format!(
            "Elysius - {:.0} FPS", ctx.time.fps()));

        //Nothing after this, pushes all the draws to the graphics card
        canvas.finish(ctx)?;
        Ok(())
    }

// 0---------------------INPUT EVENTS------------------------------------------0

    //The ggez will call events automatically for key and mouse events. 
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) -> GameResult {
        //test to make sure the game is not being zoomed out too far. 
        if self.game_scale.x < 0.2 && y == -1.0 {}
        else {
            let new_scale = self.game_scale + (y * 0.1);
            self.game_scale = new_scale;
            //println!("GameScale: {}", self.game_scale);
        }
        Ok(())
    }
    //The ggez will call this automatically to capture key_up events
    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        //add keys in here for what we want to look for. 
        match input.keycode {
            Some(KeyCode::Space) => {
                //If space, toggle the game state from play to pause
                if self.current_game_state == GameState::paused {
                        self.current_game_state = GameState::running;}
                else if self.current_game_state == GameState::running {
                    self.current_game_state = GameState::paused;}
            }
            _ => (), // Do nothing
        }
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_click_down = true;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
        Ok(())
    }

    //This gets the mouse position
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        xrel: f32,
        yrel: f32,
    ) -> GameResult {
        if self.mouse_click_down { self.mouse_click_pos = (x, y); }
        self.current_mouse_pos = (x,y);
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
