pub struct SolarObject {
    pub solar_pos: (i32, i32),
}

impl SolarObject {
    pub fn new() -> Self {
        SolarObject {
            solar_pos: (20,20),
        }
    }
    pub fn update_body_pos(&mut self) {
        self.solar_pos.0 += 1;
    }
}