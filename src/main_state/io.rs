
#[derive(PartialEq)]
pub enum MouseFocus {
    Background,
    Body(usize),
    Menu,
}

pub struct MouseState {
    pub focus: MouseFocus,
    pub pos: (f32, f32),
    pub click_down: bool,
}


// 0---------------------INPUT EVENTS------------------------------------------0



