
use ggez::{
    event::{self, MouseButton},
    graphics,
    Context, GameResult,
    input::keyboard::{KeyCode, KeyInput},
};
use super::io;
use super::super::ui;
use super::super::entities;
use super::super::user;
use super::super::globs;


#[derive(PartialEq)]
pub enum GameState {
    Running,
    Paused,
    Menu,
}



//MAIN GAME STRUCT
pub struct ElysiusMainState {
    //ECS
    pub entities: entities::Entities,
    pub entities_id: Vec<entities::EntityIndex>,
    //Structures
    pub mouse: io::Mouse,
    pub player: user::Player,
    //Game State Values
    pub first_time: bool,
    pub game_scale: glam::Vec2,
    pub player_screen_move: glam::Vec2,
    pub active_solar_system: i32,
    pub current_game_state: GameState,
    //Menu Items
    pub menus: Vec<ui::ui_comp::UIComponent> ,
}

impl ElysiusMainState {
    pub fn new(_ctx: &mut Context) -> GameResult<ElysiusMainState> {
        //This is where you can put stuff that needs to be pre-calculated

        Ok(ElysiusMainState {
            entities:       entities::Entities::new(),
            entities_id:    Vec::new(),
            mouse:          io::Mouse::new(),
            player:         user::Player::new(),
            menus:          Vec::new(),
            first_time: true,
            game_scale: glam::Vec2::new(1.0,1.0),
            player_screen_move: glam::Vec2::new(globs::SCREEN_OFFSET.0,globs::SCREEN_OFFSET.1),
            active_solar_system: 0,
            current_game_state: GameState::Running,
            
            })
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
        self.mouse.set_focus(io::MouseFocus::Background);
        self.update_menus();

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
                if entities::point_in_object(&&self.mouse.get_pos_f32(),
                    adj_pos_for_input, 
                self.entities.draw_comp[i].sprite_offset.0 as f32 * self.game_scale.x,
                ) {
                    self.mouse.set_focus(io::MouseFocus::Body(i));
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
            graphics::CanvasLoadOp::Clear([0.1, 0.1, 0.1, 1.0].into()),
        );

        //Draw ECS Ent
        for i in 0..self.entities_id.len() {
            if self.entities.solar_system_id[i] == self.active_solar_system {
                self.draw_solar_object_ecs(&mut canvas, i); 
            }
        }
        //Draw any menus on screen
        for i in 0..self.menus.len() {
            self.menus[i].draw_ui_comp(&mut canvas, &self.entities); 
        } 

        self.draw_debug_info(&mut canvas, ctx);
        //Nothing after this, pushes all the draws to the graphics card
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        self.mouse.set_click_down(false);
        Ok(())

    }  

    //Mouse button down triggers when the mouse button is pressed down, called 
    //by ggez as an update function. no need to call it yourself. 
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        self.mouse.set_click_down(true);
        //Match what the mose is focused on
        match self.mouse.get_focus() {
            io::MouseFocus::Body(id) => {
                if self.entities.ent_type[id] == entities::ObjectType::Ship {
                } else {
                    //add menu to menu stack
                    let p = glam::Vec2::new(50.0,50.0);
                    self.menus.push(
                        ui::ui_comp::UIComponent::new_menu_orbit_body_info(
                            &ctx,
                            p,
                            &self.entities,
                            id,
                        )
                    );
                }
            }
            io::MouseFocus::Background => {
                for i in 0..self.menus.len() {
                    if self.menus[i].menu_type_obi() {
                        self.menus[i].transition_out();    
                    }
                } 
            }
            io::MouseFocus::Menu => {}
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
        self.mouse.set_pos_f32((x,y));
        //this is all for a check to see if the background is dragged to move things
        if self.mouse.get_click_down() {
            match self.mouse.get_focus() {
                io::MouseFocus::Background => {
                    //self.current_mouse_pos = (x,y);
                    self.player_screen_move.x += xrel;
                    self.player_screen_move.y += yrel;
                }
                io::MouseFocus::Menu => {}
                io::MouseFocus::Body(_id) => {}
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
