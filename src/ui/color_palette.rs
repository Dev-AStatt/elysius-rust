use ggez::{graphics::Color};
pub struct ColorPalette {
    pub color_1: Color,
    pub color_2: Color,
    pub color_3: Color, 
    pub color_4: Color, 
    pub color_5: Color, 
}
impl ColorPalette {
    pub fn new() -> Self {
        ColorPalette {
            color_1: Color::from_rgba(85, 91, 110, 255),
            color_2: Color::from_rgba(137, 176, 174, 255),
            color_3: Color::from_rgba(190, 227, 219, 255),
            color_4: Color::from_rgba(250, 249, 249, 255),
            color_5: Color::from_rgba(255, 214, 186, 255),
        }
    }
}
