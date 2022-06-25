use crate::ecs::{Entities, self};
use ggez::{
    graphics::{self, Color},
    Context,
};

#[derive(PartialEq)]
pub enum MenuType {
    OrbitBodyInfo,
    ShipInfo,
    UIScreenTop,
}

struct OrbMenuPos {
    bkgr_pos: (f32, f32),
    bkgr_w: f32,
    bkgr_h: f32,
    bkgr_sec2_percent: f32,
    spr_corner: (f32,f32),
    spr_pos: glam::Vec2,
    bkgr_sec2_pos_x: f32,
    bkgr_sec2_w: f32,
    spr_w: f32,
    spr_h: f32,
    button_pos: (f32, f32),
}

impl OrbMenuPos {
    fn new() -> Self {
        //Here is all the variable locations for Maps 
        let bkgr_pos = (0.0,0.0);
        let bkgr_w = 600.0;
        let bkgr_h = 400.0;
        let bkgr_sec2_percent = 0.35;
        let spr_corner = (bkgr_w * 0.05, bkgr_h * 0.2);

        //Calculations for positions
        let bkgr_sec2_pos_x = bkgr_w * (1.0 - bkgr_sec2_percent);
        let bkgr_sec2_w = bkgr_w - bkgr_sec2_pos_x;
        let spr_w = bkgr_w *0.35;
        let spr_h = bkgr_h * 0.50;
        let button_pos = (bkgr_sec2_pos_x + (bkgr_w * 0.05), spr_corner.1);

        let spr_pos = glam::Vec2::new(spr_corner.0 + 75.0,spr_corner.1 + 75.0);

        OrbMenuPos {
            bkgr_pos,
            bkgr_w,
            bkgr_h,
            bkgr_sec2_percent,
            spr_corner,
            spr_pos,
            bkgr_sec2_pos_x,
            bkgr_sec2_w,
            spr_w,
            spr_h,
            button_pos,
          }
    }
}

struct Button {
    pos: (f32, f32),
    str_pos: (f32, f32),
    w: f32,
    h: f32,
    mesh: graphics::Mesh,
    icon: Option<graphics::Image>,
    focus: bool,
    focus_col_on: Color, 
    focus_col_off: Color, 
}

pub struct UIComponent {
    menu_type: MenuType,
    pub pos: glam::Vec2,
    pub mesh: graphics::Mesh,
    buttons: Vec<Button>,
    ent_id: usize,
}

impl UIComponent {
    pub fn new_menu_orbit_body_info(
        ctx: &Context,    
        pos_init: (f32,f32),
        ent_id: usize,
    ) -> Self {
        let menu_type = MenuType::OrbitBodyInfo;
        //Get the positions of things in the menu
        let positions = OrbMenuPos::new();
        //make position out of pos_init
        let pos = glam::Vec2::new(pos_init.0, pos_init.1);
        
        //make a new mesh for us to add things to
        let mb = &mut graphics::MeshBuilder::new();
        add_bkgr_orb_bod_to_mesh(mb, &positions);
        //build mesh
        let mesh =  graphics::Mesh::from_data(ctx, mb.build());

        //Get buttons into vector
        let mut buttons: Vec<Button> = Vec::new();
        buttons.push(get_button(positions.button_pos, ctx, None));

        let ui = UIComponent { 
                    menu_type,
                    pos,
                    mesh,
                    buttons,
                    ent_id,
                }; 
        return ui;
    }
    pub fn draw_ui_comp(
        self: &Self,
        canvas: &mut graphics::Canvas,
        ents: &Entities,
    ) {
        //Draw the background
        canvas.draw(
            &self.mesh,
            graphics::DrawParam::new().dest(self.pos)
        );
        
        match self.menu_type {
            MenuType::UIScreenTop => {}
            MenuType::ShipInfo => {}
            MenuType::OrbitBodyInfo => {
                let obi_pos = OrbMenuPos::new();
                //Draw the Sprite
                canvas.draw(
                    &ents.draw_comp[self.ent_id].sprite,
                    obi_pos.spr_pos
                );
       
 
            }
        }
   }
    

    pub fn set_pos(&mut self, pos: glam::Vec2) {
        self.pos = pos;
    }

    pub fn menu_type(&self) -> &MenuType {
        &self.menu_type
    }

    pub fn menu_type_OBI(&self) -> bool {
        if self.menu_type == MenuType::OrbitBodyInfo {
            return true;
        } else {return false;}
    }
}

struct ColorPalette {
    color_1: Color,
    color_2: Color,
    color_3: Color, 
    color_4: Color, 
    color_5: Color, 
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

//Returns a fixed sized button with an optional image to go with it. 
fn get_button(pos: (f32, f32), ctx: &Context, img: Option<graphics::Image>) -> Button {
    let btn_w = 50.0;
    let btn_h = 36.0;
    let n_str_pos; 
    match img {
        Some(_) => {
            n_str_pos = (pos.0 + 35.0, pos.1 + 2.0);
        }
        None => {
            n_str_pos = (pos.0 + 2.0, pos.1 + 2.0);    
        }
    }

    let col_palette = ColorPalette::new();
    //make mesh
    let mb = &mut graphics::MeshBuilder::new();
    mb.rounded_rectangle(
        graphics::DrawMode::fill(), 
        graphics::Rect::new(pos.0, pos.1, btn_w, btn_h),
        15.0, 
        col_palette.color_4,
    ).expect("Rec Mesh Failed");
    let mesh = graphics::Mesh::from_data(ctx, mb.build());        


    return Button {
        pos,
        str_pos: n_str_pos,
        w: 50.0,
        h: 36.0,
        mesh,
        icon: img,
        focus: false,
        focus_col_on: col_palette.color_5,
        focus_col_off: col_palette.color_4,
    };
}

//Function will add the mesh data for the background to the orbiting body mesh builder 
fn add_bkgr_orb_bod_to_mesh(
    mb: &mut graphics::MeshBuilder,
    positions: &OrbMenuPos,
) {
    let rad = 15.0;
    let color_palette = ColorPalette::new();
    //Background
    mb.rounded_rectangle(
        graphics::DrawMode::fill(), 
        graphics::Rect::new(
            positions.bkgr_pos.0,
            positions.bkgr_pos.1, 
            positions.bkgr_w,
            positions.bkgr_h),
        rad, 
        color_palette.color_1,
    ).expect("Rec Mesh Failed");
    // Second Rounded Background
    mb.rounded_rectangle(
        graphics::DrawMode::fill(), 
        graphics::Rect::new(
            positions.bkgr_sec2_pos_x, 
            positions.bkgr_pos.1,
            positions.bkgr_sec2_w,
            positions.bkgr_h),
        rad, 
        color_palette.color_2,
    ).expect("Rec Mesh Failed");
    // Boundary for sprite box
    mb.rounded_rectangle(
        graphics::DrawMode::fill(), 
        graphics::Rect::new(
            positions.spr_corner.0,
            positions.spr_corner.1,
            positions.spr_w,
            positions.spr_h),
        rad, 
        color_palette.color_2,
    ).expect("Rec Mesh Failed");

}


