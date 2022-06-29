
use ggez::{graphics, Context};
use super::color_palette;

pub struct OrbMenu {
    bkgr_pos: (f32, f32),
    bkgr_w: f32,
    bkgr_h: f32,
    spr_corner: (f32,f32),
    pub spr_pos: glam::Vec2,
    bkgr_sec2_pos_x: f32,
    bkgr_sec2_w: f32,
    spr_w: f32,
    spr_h: f32,
    pub display_item_pos: (f32, f32),
}

impl OrbMenu {
    pub fn new() -> Self {
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
        let disp_pos = (bkgr_sec2_pos_x + (bkgr_w * 0.02), spr_corner.1);

        let spr_pos = glam::Vec2::new(spr_corner.0 + 75.0,spr_corner.1 + 75.0);

        OrbMenu {
            bkgr_pos,
            bkgr_w,
            bkgr_h,
            spr_corner,
            spr_pos,
            bkgr_sec2_pos_x,
            bkgr_sec2_w,
            spr_w,
            spr_h,
            display_item_pos: disp_pos,
          }
    }
        
    //Function will add the mesh data for the background to the orbiting body mesh builder 
    pub fn get_mesh(
        &self,
        ctx: &Context,
    ) -> graphics::Mesh {
        let rad = 15.0;
        let color_palette = color_palette::ColorPalette::new();
        //Background
        let mb = &mut graphics::MeshBuilder::new();
        mb.rounded_rectangle(
            graphics::DrawMode::fill(), 
            graphics::Rect::new(
                self.bkgr_pos.0,
                self.bkgr_pos.1, 
                self.bkgr_w,
                self.bkgr_h),
            rad, 
            color_palette.color_1,
        ).expect("Rec Mesh Failed");
        // Second Rounded Background
        mb.rounded_rectangle(
            graphics::DrawMode::fill(), 
            graphics::Rect::new(
                self.bkgr_sec2_pos_x, 
                self.bkgr_pos.1,
                self.bkgr_sec2_w,
                self.bkgr_h),
            rad, 
            color_palette.color_2,
        ).expect("Rec Mesh Failed");
        // Boundary for sprite box
        mb.rounded_rectangle(
            graphics::DrawMode::fill(), 
            graphics::Rect::new(
                self.spr_corner.0,
                self.spr_corner.1,
                self.spr_w,
                self.spr_h),
            rad, 
            color_palette.color_2,
        ).expect("Rec Mesh Failed");
                //build mesh
        return graphics::Mesh::from_data(ctx, mb.build());
    }
}

