use ggez::{
    graphics::{self,Color},
    Context,
};
use super::{color_palette, disp_item::BoxSize};
use super::disp_item;
#[derive(Clone)]
pub struct Button {
    dp: disp_item::DisplayItem,
    clicked: bool,
    col_focus: Color,
    col_unfocus: Color,

}

impl Button {
    pub fn is_clicked(&self) -> bool {return self.clicked;}
    pub fn hight(&self) -> f32 {return self.dp.hight();} 
    pub fn width(&self) -> f32 {return self.dp.width();} 
    pub fn col_focus(&self) -> Color {return self.col_focus}
    pub fn col_unfocus(&self) -> Color {return self.col_unfocus}

    pub fn new(
        size_type: BoxSize,
        ctx: &Context,
        disp_string: String,
        img: Option<graphics::Image>,
    ) -> Self {
        //Make the display Item 
        let dp = disp_item::DisplayItem::new(
            glam::Vec2::new(0.0,0.0),
            size_type,
            ctx,
            disp_string,
            img,
        );
        let col = color_palette::ColorPalette::new(); 

        Button {
            dp,
            clicked: false,
            col_focus: col.color_4, 
            col_unfocus: col.color_5,
        }
    }
}


