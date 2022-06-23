#![allow(clippy::unnecessary_wraps)]

//use crate::MouseFocus::body;
use ecs::sprite_get;
use ggez::{
    event::{self, MouseButton},
    graphics::{self},
    Context, GameResult,
    input::keyboard::{KeyCode, KeyInput},
};
use glam::*;
use std::{env, path};

#[derive(PartialEq)]
enum GameState {
    Running,
    Paused,
    Menu,
}

#[derive(PartialEq)]
enum MouseFocus {
    Background,
    Body(usize),
    Menu,
}


mod ecs;
mod globs;
mod menus;

//MAIN GAME STRUCT
struct ElysiusMainState {
    //ECS
    entities: ecs::Entities,
    entities_id: Vec<ecs::EntityIndex>,
    first_time: bool,
    game_scale: glam::Vec2,
    player_screen_move: glam::Vec2,
    active_solar_system: i32,
    current_game_state: GameState,
    //mouse stuff
    current_mouse_focus: MouseFocus,
    current_mouse_pos: (f32, f32),
    mouse_click_down: bool,
    menu_trigger: (bool, usize),
    //Menu Items
    game_menus: menus::Menus,
}

impl ElysiusMainState {
    fn new(_ctx: &mut Context) -> GameResult<ElysiusMainState> {
        //This is where you can put stuff that needs to be pre-calculated

        let init_ent = ecs::Entities{
            orbit_comp: Vec::new(),
            draw_comp: Vec::new(),
            energy_comp: Vec::new(),
            solar_pos_comp: Vec::new(),
            solar_system_id: Vec::new(),
            ent_name: Vec::new(),
            ent_type: Vec::new()
        };

        Ok(ElysiusMainState {
            entities: init_ent,
            entities_id: Vec::new(),
            first_time: true,
            game_scale: glam::Vec2::new(1.0,1.0),
            player_screen_move: glam::Vec2::new(globs::SCREEN_OFFSET.0,globs::SCREEN_OFFSET.1),
            active_solar_system: 0,
            current_game_state: GameState::Running,
            //Mouse Stuff
            current_mouse_focus: MouseFocus::Background,
            current_mouse_pos: (0.0, 0.0),
            mouse_click_down: false,
            menu_trigger: (false, 0),
            game_menus: menus::Menus::new(&_ctx),
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
                    (self.entities.solar_pos_comp[orb.orbiting_ent_id].0 * self.game_scale.x) + self.player_screen_move.x,
                    (self.entities.solar_pos_comp[orb.orbiting_ent_id].1 * self.game_scale.y) + self.player_screen_move.y 
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

    fn gen_new_system(self: &mut Self, _ctx: &mut Context) {

        //First Sun
        self.entities.make_new_sun(
            &mut self.entities_id,
            sprite_get(_ctx, "/Sprite-SUN_02.png"),
            self.active_solar_system,                       
            (0.0,0.0),                                              
        );

            
        //First Planet
        self.entities.make_new_orbiting_body(
            &mut self.entities_id,
            &_ctx,
            sprite_get(_ctx, "/Sprite-Planet_01.png"),
            self.active_solar_system,                   
            0,                                     
            300                                         
        );
        //First Planet
        self.entities.make_new_orbiting_body(
            &mut self.entities_id,
            &_ctx,
            sprite_get(_ctx, "/Sprite-Moon_01.png"),
            self.active_solar_system,                
            1,                                  
            100                                         
        );
        //First Ship
        self.entities.make_new_orbiting_body(
            &mut self.entities_id,
            &_ctx,
            sprite_get(_ctx, "/Sprite-Ship_01.png"),
            self.active_solar_system,                
            1,                                  
            50,                                         
        );


        //set the flag to not run this every tick.
        self.first_time = false;
    }
}

impl event::EventHandler<ggez::GameError> for ElysiusMainState {
    //Update events go in this function
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //Create Inital test scene
        if self.first_time {
           self.gen_new_system(_ctx); 
        }
        //0----------------------GAME UPDATES----------------------------------0
        //Reset the mouse focus
        self.current_mouse_focus = MouseFocus::Background;

        for i in 0..self.entities_id.len() {
            //For all entities that are on screen
            if self.entities.solar_system_id[i] == self.active_solar_system {
                //update the final positions of entites
                self.entities.draw_comp[i].screen_pos = 
                    self.entities.get_orbit_final_pos(
                        i,
                        self.game_scale,
                        self.player_screen_move
                    );
                //update mouse focus
                let sprite_offset_scaled = (
                    self.entities.draw_comp[i].sprite_offset.0 as f32 * self.game_scale.x,
                    self.entities.draw_comp[i].sprite_offset.1 as f32 * self.game_scale.y); 
                let adj_pos_for_input = (
                    self.entities.draw_comp[i].screen_pos.x + sprite_offset_scaled.0, 
                    self.entities.draw_comp[i].screen_pos.y + sprite_offset_scaled.1
                );
                if ecs::point_in_object(&self.current_mouse_pos,
                    adj_pos_for_input, 
                self.entities.draw_comp[i].sprite_offset.0 as f32 * self.game_scale.x,
                ) {
                    self.current_mouse_focus = MouseFocus::Body(i);
                }
            
            }
        }

        

        //GameState Running
        if self.current_game_state == GameState::Running {
            self.entities.inc_orbital_body_pos(self.active_solar_system);
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
            if self.entities.solar_system_id[i] == self.active_solar_system {
                self.draw_solar_object_ecs(&mut canvas, i);
                
            }
            
        }
       
        if self.menu_trigger.0 {
            self.game_menus.draw_body_info_menu(
                &mut canvas,
                &self.entities,
                self.menu_trigger.1
            );
        } 


        //Concatinating strings is dumb
        let mut str = String::from("Tick: ");
        str.push_str(&ctx.time.ticks().to_string());
        //Draw the current tick to the screen
        canvas.draw(graphics::Text::new(str)
                    .set_scale(10.0),
                    glam::Vec2::new(0.0,990.0));

        //Draw the focus mode
        let mut focus_str = String::from("Mouse Focus: ");
        match self.current_mouse_focus {
            MouseFocus::Background => {
                focus_str.push_str("Background");
            }
            MouseFocus::Body(id) => {
                focus_str.push_str(&("Entity ".to_owned()+ &id.to_string()));
            }
            MouseFocus::Menu => {
                focus_str.push_str("Menu");
            }
        }
        canvas.draw(graphics::Text::new(focus_str)
                    .set_scale(10.0),
                    glam::Vec2::new(0.0,1000.0));

   
        //Draw the FPS counter
        ctx.gfx.set_window_title(&format!(
            "Elysius - {:.0} FPS", ctx.time.fps()));

        //Nothing after this, pushes all the draws to the graphics card
        canvas.finish(ctx)?;
        Ok(())
    }

// 0---------------------INPUT EVENTS------------------------------------------0

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        self.mouse_click_down = false;
        Ok(())
    }  

    //Mouse button down triggers when the mouse button is pressed down, called by
    //ggez as an update function. no need to call it yourself. 
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        self.mouse_click_down = true;
        
        match self.current_mouse_focus {
            MouseFocus::Body(id) => {
                //Set bool trigger for the menu popup
                self.menu_trigger = (true, id);
            }
            MouseFocus::Background => {
                //Reset bool trigger for menu popup
                self.menu_trigger.0 = false;
            }
            MouseFocus::Menu => {}
        }

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
        //set the current mouse position for the game
        self.current_mouse_pos = (x,y);
        //this is all for a check to see if the background is dragged to move things
        if self.mouse_click_down {
            match self.current_mouse_focus {
                MouseFocus::Background => {
                    //self.current_mouse_pos = (x,y);
                    self.player_screen_move.x += xrel;
                    self.player_screen_move.y += yrel;
                }
                MouseFocus::Menu => {}
                MouseFocus::Body(id) => {}
            }
        }
        Ok(()) 
    }


    //The ggez will call events automatically for key and mouse events. 
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
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
                if self.current_game_state == GameState::Paused {
                        self.current_game_state = GameState::Running;}
                else if self.current_game_state == GameState::Running {
                    self.current_game_state = GameState::Paused;}
            }
            Some(KeyCode::Z) => {
                self.player_screen_move = glam::Vec2::new(globs::SCREEN_OFFSET.0,globs::SCREEN_OFFSET.1);
            }


            _ => (), // Do nothing
        }
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
        .window_mode(ggez::conf::WindowMode::default().dimensions(globs::SCREEN_SIZE.0, globs::SCREEN_SIZE.1));

    // And finally we attempt to build the context and create the window. If it fails, we panic with the message
    // "Failed to build ggez context"
    let (mut ctx, event_loop) = cb.build()?;
    let state = ElysiusMainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
