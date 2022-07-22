use ggez::{graphics,
    Context,
};

pub struct OrbitalComponent {
    orb_ent_id: usize,
    radius: i32,
    angle: f32,
}

impl OrbitalComponent {
    //Getter Setter
    pub fn orb_ent_id(&self) -> usize {return self.orb_ent_id;}
    pub fn rad(&self) -> i32 {return self.radius;}
    pub fn angle(&self) -> f32 {self.angle}
    pub fn set_radius(&mut self, radius: i32) {self.radius = radius;}
    pub fn set_angle(&mut self, angle: f32) {self.angle = angle;}
    pub fn set_orbiting(&mut self, id: usize) {self.orb_ent_id = id;}

    //Functions
    pub fn new(radius: i32, orb_ent_id: usize) -> Self {
       //Orbit
        OrbitalComponent {orb_ent_id, radius, angle: 25.0 }
    }

    pub fn pos_adj(self: &mut Self) -> glam::Vec2 {
        //increment angle
        let mut adjustment = 20.0;   //This is what to mess around with to slow down
        adjustment = adjustment / self.rad() as f32;
        let mut new_angle = self.angle + adjustment;
        if new_angle > 360.0 {new_angle = new_angle - 360.0;}
        self.angle = new_angle;

        //calculate new position
        let unitx = (self.angle * 3.14 / 180.0).sin();
        let unity = (self.angle * 3.14 / 180.0).cos();
        let x = unitx * self.radius as f32;
        let y = unity * self.radius as f32;
        return glam::Vec2::new(x,y);
    }
}

// We may get rid of this if we can
pub struct OptionalOrbitalInputs<'a> {
    pub ctx: &'a Context,
    pub orb_ent_id: usize,
    pub orb_rad: i32,
}

