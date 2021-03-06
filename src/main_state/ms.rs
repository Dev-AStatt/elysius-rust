
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
use super::game_state;
use super::event_system::{EventType, EventSystem};     

//MAIN GAME STRUCT
pub struct ElysiusMainState {
    //ECS
    pub entities: entities::Entities,
    pub entities_id: Vec<entities::EntityIndex>,
    //Structures
    pub mouse: io::Mouse,
    pub player: user::Player,
    pub state: game_state::GameState,
    //Menu Items
    pub menus: Vec<ui::ui_comp::UIComponent> ,
    pub events : EventSystem,
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
            state:          game_state::GameState::new(), 
            events:         EventSystem::new(),
        })
    }
}


impl event::EventHandler<ggez::GameError> for ElysiusMainState {
    
    //Update events go in this function
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //Create Inital test scene
        if self.state.first_time() {
            self.gen_new_system(ctx); 
        }
        //0----------------------GAME UPDATES----------------------------------0
        self.update_mouse();
        self.update_menus(ctx);
        self.entities.update(&self.entities_id, &self.state, &mut self.events);

        self.events.clear_events();         //Make sure to clear events last
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1, 0.1, 0.1, 1.0].into()),
        );

        //Draw Entities
        self.entities.draw_objects(&mut canvas, &self.entities_id, &self.state);
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
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        self.events.new_event_ez(EventType::LeftMouseDown); 
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
            if self.mouse.get_focus() == io::MouseFocus::Background {
                let n_rel = glam::Vec2::new(xrel,yrel);
                self.state.adj_player_screen_offset_pos(n_rel);
           }
       }
        Ok(()) 
    }
    
    
    //The ggez will call events automatically for key and mouse events. 
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        //test to make sure the game is not being zoomed out too far. 
        if self.state.scale().x < 0.2 && y == -1.0 {}
        else {
            let new_scale = self.state.scale() + (y * 0.1);
            self.state.set_scale(new_scale);
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
                if self.state.if_state_is(game_state::StateType::Paused) {
                        self.state.set_state_type(game_state::StateType::Running);}
                else if self.state.if_state_is(game_state::StateType::Running) {
                    self.state.set_state_type(game_state::StateType::Paused);}
                }
            Some(KeyCode::Z) => {
                self.state.set_scale(self.state.screen_offset());
            }
            _ => (), // Do nothing
        }
        Ok(())
    }
}
