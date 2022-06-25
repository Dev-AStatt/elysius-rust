use crate::ecs::{Entities, self};
use ggez::{
    graphics::{self, Color},
    Context,
};

enum MenuType {
    OrbitBodyInfo,
    ShipInfo,
    UIScreenTop,
}

struct Button {
    pos: (f32, f32),
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
    pos: (f32, f32),
    pub mesh: graphics::Mesh,
    buttons: Vec<Button>,
}

impl UIComponent {
    pub fn new_menu_orbit_body_info(
        ctx: &Context,    
        pos_init: (f32,f32)
    ) -> Self {
        let menu_type = MenuType::OrbitBodyInfo;
       
        
        //make a new mesh for us to add things to
        let mb = &mut graphics::MeshBuilder::new();
        add_bkgr_orb_bod_to_mesh(mb);
        //build mesh
        let mesh =  graphics::Mesh::from_data(ctx, mb.build());
        let ui = UIComponent { 
                    menu_type,
                    pos: pos_init,
                    mesh,
                    buttons: Vec::new(),
                }; 



        return ui;
    }
    //Returns a fixed sized button with an optional image to go with it. 
    fn get_button(pos: (f32, f32), ctx: &Context, img: Option<graphics::Image>) -> Button {
        let btn_w = 50.0;
        let btn_h = 36.0;
        
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
            w: 50.0,
            h: 36.0,
            mesh,
            icon: img,
            focus: false,
            focus_col_on: col_palette.color_5,
            focus_col_off: col_palette.color_4,
        };
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

//Function will add the mesh data for the background to the orbiting body mesh builder 
fn add_bkgr_orb_bod_to_mesh(
    mb: &mut graphics::MeshBuilder,
) {
    let rad = 15.0;
    //Here is all the variable locations for Maps 
    let bkgr_pos = (0.0,0.0);
    let bkgr_w = 600.0;
    let bkgr_h = 400.0;
    let bkgr_sec2_percent = 0.35;
    let spr_corner = (bkgr_w * 0.05, bkgr_h * 0.2);

    //Calculations for positions
    let color_palette = ColorPalette::new();
    let bkgr_sec2_pos_x = bkgr_w * (1.0 - bkgr_sec2_percent);
    let bkgr_sec2_w = bkgr_w - bkgr_sec2_pos_x;
    let spr_w = bkgr_w *0.35;
    let spr_h = bkgr_h * 0.50;

    //Background
    mb.rounded_rectangle(
        graphics::DrawMode::fill(), 
        graphics::Rect::new(bkgr_pos.0, bkgr_pos.1, bkgr_w,bkgr_h),
        rad, 
        color_palette.color_1,
    ).expect("Rec Mesh Failed");
    // Second Rounded Background
    mb.rounded_rectangle(
        graphics::DrawMode::fill(), 
        graphics::Rect::new(bkgr_sec2_pos_x, bkgr_pos.1,bkgr_sec2_w,bkgr_h),
        rad, 
        color_palette.color_2,
    ).expect("Rec Mesh Failed");
    // Boundary for sprite box
    mb.rounded_rectangle(
        graphics::DrawMode::fill(), 
        graphics::Rect::new(spr_corner.0,spr_corner.1,spr_w,spr_h),
        rad, 
        color_palette.color_2,
    ).expect("Rec Mesh Failed");

}


