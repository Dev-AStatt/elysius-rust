pub struct SolarSystem {
    pub intersteller_pos: (i32, i32),
    pub sun: SolarObject,
    pub orbiting_bodies: Vec<SolarObject>

}

impl SolarSystem {
    pub fn new(inter_pos: (i32,i32)) -> Self {
        SolarSystem {
            intersteller_pos: inter_pos,
            sun: SolarObject::new((100,50),6, (255,255,0)),
            orbiting_bodies: vec!(SolarObject::new((100,25),2, (255,255,255))),
        }
    }
    pub fn numb_of_bodies(self: &Self) -> i32 {
        return self.orbiting_bodies.len() as i32;
    }
}

pub struct SolarObject {
    pub solar_pos: (i32, i32),
    pub color: (u8, u8, u8), //touple for (r, g, b)
    pub size: i32,
    pub moons: Vec<SolarObject>,


}

impl SolarObject {
    //Function input (r,g,b)
    pub fn new(new_pos: (i32,i32),new_size: i32, col: (u8,u8,u8)) -> Self {
        SolarObject {
            solar_pos: new_pos,
            size: new_size,
            color: col,
            moons: Vec::new(),

        }
    }
    pub fn update_body_pos(&mut self) {
        self.solar_pos.0 += 1;
    }

    pub fn add_orbiting_body(&mut self, obj: SolarObject) {
        self.moons.push(obj);
    }

    pub fn are_there_moons(self: &Self) -> bool {
        if self.moons.len() == 0 { return true;}
        return false;
    }
//
}