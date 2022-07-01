use std::ops::Div;

pub enum TransitionType {
    Slide,
    Pop
}

pub struct Transition {
    in_transition: bool,
    t_type: TransitionType,
    pos_end: glam::Vec2,
    pos: glam::Vec2,
    dpt: glam::Vec2,
    arrived: bool,

}

impl Transition {
    pub fn new(
        t_type: TransitionType,
        pos_start: glam::Vec2,
        pos_end: glam::Vec2
    ) -> Self {
        let dpt = Transition::get_dist_per_tick(&t_type, pos_start, pos_end, 60.0);
        Transition {
            in_transition: true,
            t_type,
            pos_end,
            pos: pos_start,
            dpt,
            arrived: false,
        }
    }


    pub fn is_in_transition(&self) -> bool {return self.in_transition;}

    pub fn inc_transition(self: &mut Self) {
        if self.arived_at_dest() {
            self.arrived = true;
        } else {
            //Incriment Position
            self.pos += self.dpt;
        }
    }

    fn arived_at_dest(&self) -> bool {
        let t_v = f32::min(self.dpt.x, self.dpt.y);
        if (self.pos.x - self.pos_end.x).powi(2) + (self.pos.y - self.pos_end.y).powi(2) < t_v.powi(2) {
            return true;
        } else {return false;}
    }   


    fn get_dist_per_tick(
        t_t: &TransitionType,
        pos: glam::Vec2, 
        p_end: glam::Vec2, 
        ticks: f32
    ) -> glam::Vec2 {
        let mut dpt = glam::Vec2::new(0.0,0.0);
        match t_t {
            TransitionType::Pop => {}
            TransitionType::Slide => {
                //Get the total distance we need to move
                let total = p_end - pos; 
                dpt = total.div(ticks);
            }
        }
        return dpt;
   }
}







//0----------------------------TESTS-------------------------------------------0


#[cfg(test)]
mod tests {
    use crate::ui::transtions::{Transition, TransitionType};

    //Test is designed to create a transition and push it through a transition phase
    //If the transition phase is caught by the check end. Pass 
    #[test]
    fn test_inc_pos() {
        let mut t_1 = Transition::new(
            TransitionType::Slide,
            glam::Vec2::new(0.0,0.0),
            glam::Vec2::new(10.0,10.0),
        );
        while t_1.arrived == false {
            t_1.inc_transition();
        }
        let mut t_2 = Transition::new(
            TransitionType::Slide,
            glam::Vec2::new(10.0,10.0),
            glam::Vec2::new(0.0,0.0),
        );
        while t_2.arrived == false {
            t_2.inc_transition();
        }
        assert!(t_1.arrived && t_2.arrived);
    }
}