use std::ops::Div;

pub enum TransitionType {
    Slide,
    Pop
}

pub struct Transition {
    in_transition: bool,
    transition_type: TransitionType,
    pos_start: glam::Vec2,
    pos_end: glam::Vec2,
    pos: glam::Vec2,
    dpt: glam::Vec2,

}

impl Transition {
    pub fn new(
        transition_type: TransitionType,
        pos_start: glam::Vec2,
        pos_end: glam::Vec2
    ) -> Self {
        let mut t = Transition {
            in_transition: true,
            transition_type,
            pos_start,
            pos_end,
            dpt: glam::Vec2::new(0.0,0.0),
            pos: pos_start,
        };
        t.get_dist_per_tick(2.0); 
        return t;
    }


    pub fn is_in_transition(&self) -> bool {return self.in_transition;}

    pub fn inc_transition(self: &mut Self) {

    }

    fn arived_at_dest(&self) -> bool {
        
        //if position is within plus or minus dpt of the end position return true
        //get Square
        let area_p1 = self.pos_end + self.dpt;
        let area_p2 = self.pos_end - self.dpt;
        


        return false;
    }

    fn point_in_square(s_1: glam::Vec2, s_2: glam::Vec2, p: glam::Vec2) -> bool {
        return true;
    }


    fn get_dist_per_tick(self: &mut Self, ticks: f32) {

        match self.transition_type {
            TransitionType::Pop => {
                self.dpt = glam::Vec2::new(0.0,0.0);
            }
            TransitionType::Slide => {
                //Get the total distance we need to move
                let total = self.pos_end - self.pos_start; 
                self.dpt = total.div(ticks);
            }
        }
   }
}





//0----------------------------TESTS-------------------------------------------0


#[cfg(test)]
mod tests {
    use crate::ui::transtions::{Transition, TransitionType};

    #[test]
    fn get_dist_per_tick() {
        let mut t = Transition::new(
            TransitionType::Slide,
            glam::Vec2::new(2.0,2.0),
            glam::Vec2::new(122.0,122.0),
        );
        let answer = glam::Vec2::new(2.0,2.0);
        assert_eq!(true , true);
    }

    #[test]
    fn test_normlize_vector() {
        let mut t = Transition::new(
            TransitionType::Slide,
            glam::Vec2::new(2.0,2.0),
            glam::Vec2::new(122.0,122.0),
        );
        t.arived_at_dest();
        assert!(true);
    }

}