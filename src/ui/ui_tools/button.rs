use ggez::{
    graphics::{self,Color},
    Context,
};
use crate::{utilities, main_state::event_system};

use super::{color_palette, disp_item::BoxSize};
use super::disp_item;
#[derive(Clone)]
pub struct Button {
    dp: disp_item::DisplayItem,
    mouse_over: bool,
    col_focus: Color,
    event: event_system::Event,

}

impl Button {
    pub fn mouse_over(&self) -> bool {return self.mouse_over;}
    pub fn hight(&self) -> f32 {return self.dp.hight();} 
    pub fn width(&self) -> f32 {return self.dp.width();} 
    pub fn size(&self) -> glam::Vec2 {return self.dp.box_size().size(); }
    pub fn col_focus(&self) -> Color {return self.col_focus}
    pub fn rel_pos(&self) -> glam::Vec2 {return self.dp.rel_pos()}
    pub fn linked_event(&self) -> event_system::Event {return self.event;}
    //Sets
    pub fn set_mouse_over(self: &mut Self) {self.mouse_over = true;}

    pub fn new(
        size_type: BoxSize,
        pos: glam::Vec2,
        ctx: &Context,
        disp_string: String,
        img: Option<graphics::Image>,
        event: event_system::Event,
    ) -> Self {
        //Make the display Item 
        let dp = disp_item::DisplayItem::new(
            pos,
            size_type,
            ctx,
            disp_string,
            img,
            None,
        );
        let col = color_palette::ColorPalette::new(); 

        Button {
            dp,
            mouse_over: false,
            col_focus: col.color_3, 
            event,
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, menu_pos: glam::Vec2) {
        if self.mouse_over {
            self.dp.draw_with_color(canvas, menu_pos, Some(self.col_focus));
        } else {
            self.dp.draw_with_color(canvas, menu_pos, None);
        } 
    }

    pub fn mouse_over_bttn(&self, menu_pos: glam::Vec2, mouse_pos: glam::Vec2
    ) -> bool {
        let tl = menu_pos + self.rel_pos();
        let br = tl + self.size(); 
        if utilities::point_in_square(tl, br, mouse_pos) {return true;}
        else {return false;} 
    }

    pub fn update(
        self: &mut Self, 
        menu_pos: glam::Vec2, 
        mouse_pos: glam::Vec2,
        events: &mut event_system::EventSystem,
    ) {
        if self.mouse_over_bttn(menu_pos,mouse_pos) {
            self.mouse_over = true;
            self.event_check(events);
        } else {
            self.mouse_over = false;
        }
    }

    fn event_check(self: &mut Self, events: & mut event_system::EventSystem) {
        if events.check_event(event_system::EventType::LeftMouseDown) {
            events.new_event_from(self.event);
            println!("Event Button Click");
        } 
    }


}


