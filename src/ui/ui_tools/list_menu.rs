use ggez::{graphics, Context};
use super::color_palette;
use super::button;
use super::super::ui_comp;

pub struct ListMenu {
    bkgr_w: f32,
    bkgr_h: f32,
    buttons: Vec<button::Button>,

}


impl ListMenu {
    //Getters and Setters
    

    pub fn new(m_type: ui_comp::MenuType) -> Self {
       
        let buttons: Vec<button::Button> = Vec::new();
        if m_type == ui_comp::MenuType::ShipOptions {

        }


        ListMenu {
            bkgr_w,
            bkgr_h,
            buttons,
        }
    }

}


