use ggez::graphics;


#[derive(PartialEq, Copy, Clone)]
pub enum MouseFocus {
    Background,
    Body(usize),
    Menu,
}

pub struct Mouse {
    focus: MouseFocus,
    pos: (f32, f32),
    click_down: bool,
}
//Getters and Setters
impl Mouse {
    pub fn set_focus(self: &mut Self, f: MouseFocus) {self.focus = f;}
    pub fn set_pos_f32(self: &mut Self, p: (f32,f32)) {self.pos = p;}
    pub fn set_pos_vec2(self: &mut Self, p: glam::Vec2) {self.pos = (p.x,p.y);}
    pub fn set_click_down(self: &mut Self, b: bool) {self.click_down = b;}

    pub fn get_click_down(&self) -> bool {return self.click_down;}
    pub fn get_pos_f32(&self) -> (f32,f32) {return self.pos;}
    pub fn get_pos_vec2(&self) -> glam::Vec2 {return glam::Vec2::new(self.pos.0,self.pos.1);}
    pub fn get_focus(&self) -> MouseFocus {return self.focus;}
}


impl Mouse {
    pub fn new() -> Self {
        Mouse { 
            focus: MouseFocus::Background,
            pos: (0.0,0.0), 
            click_down: false 
        }
    }
    pub fn get_focus_as_string(&self) -> String {
        let mut str = String::from("Mouse Focus: ");
        match self.focus {
            MouseFocus::Background => {
                str.push_str("Background");
            }
            MouseFocus::Body(id) => {
                str.push_str(&("Entity ".to_owned()+ &id.to_string()));
            }
            MouseFocus::Menu => {
                str.push_str("Menu");
            }
        }
        return str;
    }

}


