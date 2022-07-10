use ggez::{graphics,
    Context,
};

pub struct OrbitalComponent {
    orb_ent_id: usize,
    radius: i32,
    angle: f32,
    orbit_circle: graphics::Mesh,
}

impl OrbitalComponent {
    //Getter Setter
    pub fn orb_ent_id(&self) -> usize {return self.orb_ent_id;}
    pub fn rad(&self) -> i32 {return self.radius;}
    pub fn angle(&self) -> f32 {self.angle}
    pub fn orbit_circle(&self) -> &graphics::Mesh {&self.orbit_circle}
    pub fn set_radius(&mut self, radius: i32) {self.radius = radius;}
    pub fn set_angle(&mut self, angle: f32) {self.angle = angle;}
    pub fn set_orbiting(&mut self, id: usize) {self.orb_ent_id = id;}

    //Functions
    pub fn new(
        ctx: &Context,
        radius: i32,
        orb_ent_id: usize,
    ) -> Self {
        //get a new meshbuilder to make our circle
        let mb = &mut graphics::MeshBuilder::new();
        //get our new circle
        mb.circle(graphics::DrawMode::stroke(1.0),
            glam::vec2(0.0,0.0), //dest.0,dest.1
            radius as f32,
            2.0,
            graphics::Color::WHITE).expect("ecs new planet mesh error");

        let orbit_circle = graphics::Mesh::from_data(ctx, mb.build());
        //Orbit
        OrbitalComponent {
            orb_ent_id,
            radius,
            angle: 25.0,
            orbit_circle,
        }
    }

    pub fn pos_adj(self: &mut Self) -> glam::Vec2 {
        //increment angle
        let adjustment = 0.1;   //This is what to mess around with to slow down
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

