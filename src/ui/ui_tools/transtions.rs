
#[derive(PartialEq, Copy,Clone)]
pub enum InOrOut {
    IN,
    OUT,
}

pub enum TransitionType {
    Slide,
    Pop
}

pub enum Speed {
    Normal,
    Fast,
}
impl Speed {
    fn get_speed(&self) -> f32 {
        match self {
            Speed::Normal => {return 20.0;}
            Speed::Fast   => {return 5.0;}
        }
    }
}    


pub struct Transition {
    in_transition: bool,
    t_type: TransitionType,
    pos_end: glam::Vec2,
    pos: glam::Vec2,
    dpt: glam::Vec2,
    t_io: InOrOut,
    speed: Speed,
}

impl Transition {
    pub fn new(
        t_type: TransitionType,
        pos_start: glam::Vec2,
        pos_end: glam::Vec2,
        t_io: InOrOut,
        speed: Speed,
    ) -> Self {
        let dpt = Transition::get_dist_per_tick(
            &t_type, 
            pos_start, 
            pos_end, 
            &speed,
        );
        Transition {
            in_transition: true,
            t_type,
            pos_end,
            pos: pos_start,
            dpt,
            t_io,
            speed,
        }
    }
    pub fn get_pos(&self) -> glam::Vec2 {return self.pos;}
    pub fn is_in_transition(&self) -> bool {return self.in_transition;}
    pub fn in_or_out(&self) -> InOrOut {return self.t_io;  }

    pub fn inc_transition(self: &mut Self) {
        if self.arived_at_dest() {
            self.in_transition = false;
        } else {
            //Incriment Position
            self.pos += self.dpt;
        }
    }

    fn arived_at_dest(&self) -> bool {
        let mut t_v = f32::min(self.dpt.x, self.dpt.y);
        if t_v == 0.0 {
            t_v = 0.1;
        }
        if (self.pos.x - self.pos_end.x).powi(2) + (self.pos.y - self.pos_end.y).powi(2) < t_v.powi(2) {
            return true;
        } else {return false;}
    }   

    fn get_dist_per_tick(
        t_t: &TransitionType,
        pos: glam::Vec2, 
        p_end: glam::Vec2, 
        s: &Speed,
    ) -> glam::Vec2 {
        let mut dpt = glam::Vec2::new(0.0,0.0);
        match t_t {
            TransitionType::Pop => {}
            TransitionType::Slide => {
                //Get the total distance we need to move
                let total = p_end - pos; 
                dpt = std::ops::Div::div(total, s.get_speed());
            }
        }
        return dpt;
   }
}







//0----------------------------TESTS-------------------------------------------0
