use ggez::{
    graphics,
    Context,
};

use super::ms;


impl ms::ElysiusMainState {
    //Draw function for solar objects and their rings
    pub fn draw_solar_object_ecs(
        self: &Self,
        canvas: &mut graphics::Canvas,
        ent_id: usize
    ) {
        //if there is some orb component, then 
        if let Some(ref orb) = &self.entities.orbit_comp[ent_id] {
            //get the final position of the circle
            let circle_pos = (
                //self.entities.solar_pos_comp[orb.orb_ent_id()]
                self.entities.position_comp[orb.orb_ent_id()].solar_pos()
                * self.state.scale()
                ) + self.state.player_screen_offset_pos();

            //Draw the circle
            canvas.draw(orb.orbit_circle(), 
                graphics::DrawParam::new()
                    .scale(self.state.scale())
                    .dest(circle_pos)
            ); 
        }
        //Draw the sprite
        canvas.draw(self.entities.draw_comp[ent_id].sprite(),
            graphics::DrawParam::new()
                .dest(self.entities.draw_comp[ent_id].screen_pos())
                .scale(self.state.scale())
        );
    }

    pub fn draw_debug_info(
        &self,
        canvas: &mut graphics::Canvas,
        ctx: &mut Context,
    ) {        //Concatinating strings is dumb
        let mut str = String::from("Tick: ");
        str.push_str(&ctx.time.ticks().to_string());
        //Draw the current tick to the screen
        canvas.draw(graphics::Text::new(str)
                    .set_scale(10.0),
                    glam::Vec2::new(0.0,990.0));

        let mut str = String::from("Menus In Stack: ");
        str.push_str(&self.menus.len().to_string());
        //Draw the current tick to the screen
        canvas.draw(graphics::Text::new(str)
                    .set_scale(10.0),
                    glam::Vec2::new(0.0,980.0));

        //Draw the focus mode
        canvas.draw(
            graphics::Text::new(self.mouse.get_focus_as_string())
            .set_scale(10.0),
            glam::Vec2::new(0.0,1000.0)
        );


        //Draw the FPS counter
        ctx.gfx.set_window_title(&format!(
            "Elysius - {:.0} FPS", ctx.time.fps()));



    }

}


