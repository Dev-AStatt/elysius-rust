use ggez::{graphics, Context};
use super::color_palette;
use super::button;
use super::super::ui_comp;
use super::disp_item;

pub struct ListMenu {
    hw: glam::Vec2,
    buttons: Vec<button::Button>,
    title: disp_item::DisplayItem,

}


impl ListMenu {
    //Getters and Setters
    pub fn buttons(&self) -> Vec<button::Button> {return self.buttons;}
    pub fn title(&self) -> disp_item::DisplayItem {return self.title;}
    pub fn hight(&self) -> f32 {return self.hw.x;}
    pub fn width(&self) -> f32 {return self.hw.y;}
    pub fn size(&self) -> glam::Vec2 {return self.hw;}

    pub fn new(m_type: ui_comp::MenuType, ctx: &Context) -> Self {
        let mut buttons: Vec<button::Button> = Vec::new();
        let title = disp_item::DisplayItem::new(
            glam::Vec2::new(0.0,0.0),
            disp_item::BoxSize::Small,
            ctx,
            "Title".to_string(),
            None,
        );
        if m_type == ui_comp::MenuType::ShipOptions {
            buttons = get_test_vect(ctx); 
        } 
                 

        ListMenu {
            hw: calc_bkgr_pos(&buttons, &title),
            buttons,
            title,
        }
    }
}

fn calc_bkgr_pos(
    buttons: &Vec<button::Button>, 
    title: &disp_item::DisplayItem
) -> glam::Vec2 {
    //Buffer Between buttons
    let buff: f32 = 15.0;
    let mut total_hight: f32;

    total_hight = buff + title.hight() + buff;
    
    for i in 0..buttons.len() {
        total_hight += buttons[i].hight();
        total_hight += buff;
    }

    return glam::Vec2::new(total_hight, title.width())
}



//function for testing filling a vector of junk values
fn get_test_vect(ctx: &Context) -> Vec<button::Button> {
    let buttons: Vec<button::Button> = Vec::new();
        let buttons = Vec::new();
        buttons.push(button::Button::new(
            super::disp_item::BoxSize::Small,
            ctx,
            "Test 1".to_string(),
            None,
        ));
        buttons.push(button::Button::new(
            super::disp_item::BoxSize::Small,
            ctx,
            "Test 2".to_string(),
            None,
        ));
     buttons.push(button::Button::new(
            super::disp_item::BoxSize::Small,
            ctx,
            "Test 3".to_string(),
            None,
        ));


    return buttons;
}













