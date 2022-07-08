use ggez::input::mouse::position;
use ggez::{graphics, Context};
use super::color_palette;
use super::button;
use super::super::ui_comp;
use super::disp_item;

pub struct ListMenu {
    hw: glam::Vec2,
    buttons: Vec<button::Button>,
    title: disp_item::DisplayItem,
    positions: Vec<glam::Vec2>

}

impl ListMenu {
    //Getters and Setters
    pub fn buttons(&self) -> Vec<button::Button> {return self.buttons.clone();}
    pub fn title(&self) -> disp_item::DisplayItem {return self.title.clone();}
    pub fn hight(&self) -> f32 {return self.hw.x;}
    pub fn width(&self) -> f32 {return self.hw.y;}
    pub fn size(&self) -> glam::Vec2 {return self.hw;}
    

    pub fn new(m_type: ui_comp::MenuType, ctx: &Context) -> Self {
        let buttons: Vec<button::Button> = Vec::new();
        let mut positions = Vec::new();
        //add the title
        let title = disp_item::DisplayItem::new(
            glam::Vec2::new(0.0,0.0),
            disp_item::BoxSize::Small,
            ctx,
            "Title".to_string(),
            None,
        );
        //add the title position to it
        positions.push(glam::Vec2::new(15.0,15.0));
        
        let mut l = ListMenu {
            hw: glam::Vec2::new(0.0,0.0),
            buttons,
            title,
            positions,
        };
        if m_type == ui_comp::MenuType::ShipOptions {
            l.build_ship_menu(ctx);
        }
        return l;
    }
    
    fn build_ship_menu(self: &mut Self, ctx: &Context) {
        //For each button to add
        for i in 0..2 {
            self.add_button(ctx, "Text Test".to_string());
        }        
    }

    fn add_button(self: &mut Self, ctx: &Context, text: String) {
        //get next position
        if let Some(last) = self.positions.last() {
            let next_pos = glam::Vec2::new(last.x,last.y + 50.0 + 15.0);
            self.positions.push(next_pos);
            self.buttons.push(
              button::Button::new(
                disp_item::BoxSize::Small,
                next_pos,
                ctx,
                text,
                None,
              )  
            );
        } 
    }

    pub fn mesh(&self, ctx: &Context) -> graphics::Mesh {
        let rad = 15.0;
        let color_palette = color_palette::ColorPalette::new();
        let mb = &mut graphics::MeshBuilder::new();
        mb.rounded_rectangle(
            graphics::DrawMode::fill(), 
            graphics::Rect::new(
                0.0,
                0.0, 
                self.hw.x,
                self.hw.y),
            rad, 
            color_palette.color_1,
        ).expect("list menu Mesh Failed");

        return graphics::Mesh::from_data(ctx, mb.build());
    }

}













