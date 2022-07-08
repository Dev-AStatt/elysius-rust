use ggez::{
    graphics::{self,Color},
    Context,
};
use crate::utilities;

use super::{color_palette, disp_item::BoxSize};
use super::disp_item;
#[derive(Clone)]
pub struct Button {
    dp: disp_item::DisplayItem,
    mouse_over: bool,
    col_focus: Color,
    col_unfocus: Color,

}

impl Button {
    pub fn mouse_over(&self) -> bool {return self.mouse_over;}
    pub fn hight(&self) -> f32 {return self.dp.hight();} 
    pub fn width(&self) -> f32 {return self.dp.width();} 
    pub fn size(&self) -> glam::Vec2 {return self.dp.box_size().size(); }
    pub fn col_focus(&self) -> Color {return self.col_focus}
    pub fn col_unfocus(&self) -> Color {return self.col_unfocus}
    pub fn rel_pos(&self) -> glam::Vec2 {return self.dp.rel_pos()}
    //Sets
    pub fn set_mouse_over(self: &mut Self) {self.mouse_over = true;}

    pub fn new(
        size_type: BoxSize,
        pos: glam::Vec2,
        ctx: &Context,
        disp_string: String,
        img: Option<graphics::Image>,
    ) -> Self {
        //Make the display Item 
        let dp = disp_item::DisplayItem::new(
            pos,
            size_type,
            ctx,
            disp_string,
            img,
        );
        let col = color_palette::ColorPalette::new(); 

        Button {
            dp,
            mouse_over: false,
            col_focus: col.color_4, 
            col_unfocus: col.color_5,
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, menu_pos: glam::Vec2) {
        self.dp.draw(canvas, menu_pos);
    }

    pub fn mouse_over_bttn(&self, menu_pos: glam::Vec2, mouse_pos: glam::Vec2
    ) -> bool {
        let tl = menu_pos + self.rel_pos();
        let br = tl + self.size(); 
        if utilities::point_in_square(tl, br, mouse_pos) {return true;}
        else {return false;} 
    }

    pub fn update(self: &mut Self, menu_pos: glam::Vec2, mouse_pos: glam::Vec2) {
        if self.mouse_over_bttn(menu_pos,mouse_pos) {self.mouse_over = true;}
        else {self.mouse_over = false;}
    }



}


