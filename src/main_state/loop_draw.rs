use ggez::{
    graphics,
};

use super::ms;

impl ms::ElysiusMainState {
    //Draw function for solar objects and their rings
    pub fn draw_solar_object_ecs(
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
}


