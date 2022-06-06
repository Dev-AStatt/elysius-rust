pub struct SolarSystem {
    intersteller_pos: (i32, i32),
    pub sun: SolarObject,
    pub orbiting_bodies: Vec<SolarObject>

}

impl SolarSystem {
    pub fn new(inter_pos: (i32,i32)) -> Self {
        SolarSystem {
            intersteller_pos: inter_pos,
            //Create the sun object
            sun: SolarObject::new(0,6, (255,255,0)),
            orbiting_bodies: vec!(SolarObject::new(25,2, (255,255,255))),
        }
    }
    //Returns the length of orbital bodies vector
    pub fn numb_of_bodies(self: &Self) -> i32 {
        return self.orbiting_bodies.len() as i32;
    }

    pub fn update_bodies(&mut self) {
        for i in 0..self.numb_of_bodies() {
            self.orbiting_bodies[i as usize].update_body_pos();
        }
    }

}

pub struct SolarObject {
    pub solar_pos: (i32, i32),
    pub color: (u8, u8, u8), //touple for (r, g, b)
    pub size: i32,
    pub orbit_angle: i32,
    pub orbit_radius: i32,
    pub moons: Vec<SolarObject>,


}

impl SolarObject {
    //Function input (r,g,b)
    pub fn new(orb_rad: i32,new_size: i32, col: (u8,u8,u8)) -> Self {
        //calculate origin position, if sun fix position
        let mut new_pos: (i32,i32);
        if orb_rad == 0 {new_pos = (100, 50);}
        else {new_pos = (100, orb_rad);}
        SolarObject {
            size: new_size,
            color: col,
            orbit_angle: 0,
            orbit_radius: orb_rad,
            moons: Vec::new(),
            solar_pos: new_pos,
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

}