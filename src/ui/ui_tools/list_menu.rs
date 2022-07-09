use ggez::{graphics, Context};
use crate::main_state::event_system;

use super::color_palette;
use super::button;
use super::super::ui_comp;
use super::disp_item;

pub struct ListMenu {
    hw: glam::Vec2,
    buttons: Vec<button::Button>,
    title: disp_item::DisplayItem,
    positions: Vec<glam::Vec2>,
    bttn_gap: f32,
    size: disp_item::BoxSize,
    linked_ent: usize,

}

impl ListMenu {
    //Getters and Setters
    pub fn buttons(&self) -> Vec<button::Button> {return self.buttons.clone();}
    pub fn title(&self) -> disp_item::DisplayItem {return self.title.clone();}
    pub fn hight(&self) -> f32 {return self.hw.x;}
    pub fn width(&self) -> f32 {return self.hw.y;}
    pub fn size(&self) -> glam::Vec2 {return self.hw;}
    

    pub fn new(m_type: ui_comp::MenuType, ctx: &Context, title_str: String, linked_ent: usize) -> Self {
        let buttons: Vec<button::Button> = Vec::new();
        let mut positions = Vec::new();
        //add the title position to it
        positions.push(glam::Vec2::new(15.0,15.0));
        
        let title = disp_item::DisplayItem::new(
            positions[0],
            disp_item::BoxSize::Small,
            ctx,
            title_str, 
            None,
            Some(color_palette::ColorPalette::new().color_1),
        );
        let mut l = ListMenu {
            hw: glam::Vec2::new(0.0,0.0),
            buttons,
            title,
            positions,
            bttn_gap: 15.0,
            size: disp_item::BoxSize::Large,
            linked_ent,
        };
        if m_type == ui_comp::MenuType::ShipOptions {
            l.build_ship_menu(ctx);
        }
        return l;
    }
    
    fn build_ship_menu(self: &mut Self, ctx: &Context) {
        //Add the first Option
        self.add_button(ctx,"Move Ship".to_string());
        self.add_button(ctx,"Option 2".to_string());
        self.hw = self.get_hw(); 
    }

    fn get_hw(self: &mut Self) -> glam::Vec2 {
        let mut h: f32 = 0.0;
        let w: f32 = self.size.get_width() + 30.0;
        if let Some(last) = self.positions.last() {
            h = last.y + 50.0 + 15.0;
        }
        return glam::Vec2::new(w,h);
    }

    fn add_button(
        self: &mut Self, 
        ctx: &Context, 
        text: String, 
        ) {
        //check if the positions vector is not empty
        if let Some(last) = self.positions.last() {
            //get next position
            let next_pos = glam::Vec2::new(
                last.x,
                last.y + self.size.get_hight() + self.bttn_gap
            );
            self.positions.push(next_pos);
            //Make generic event
            let linked_event = event_system::Event::new(
                event_system::ElysiusEventType::InitShipTransfer, 
                Some(self.linked_ent), None);

            //create button
            self.buttons.push(
              button::Button::new(
                self.size,
                next_pos,
                ctx,
                text,
                None,
                linked_event,
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













