
use ggez::graphics;

use super::color_palette;
pub struct OrbMenuPos {
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

impl OrbMenuPos {
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

        OrbMenuPos {
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
}


//Function will add the mesh data for the background to the orbiting body mesh builder 
pub fn add_bkgr_orb_bod_to_mesh(
    mb: &mut graphics::MeshBuilder,
    positions: &OrbMenuPos,
) {
    let rad = 15.0;
    let color_palette = color_palette::ColorPalette::new();
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


