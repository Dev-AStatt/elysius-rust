use ggez::{
    graphics::{self,Color},
    Context,
};
use super::color_palette;
use super::disp_item;

struct Button {
    dp: disp_item::DisplayItem,
    clicked: bool,
    col_focus: Color,
    col_unfocus: Color,

}

impl Button {
    pub fn is_clicked(&self) -> bool {return self.clicked;}
     
}


