use ggez::{graphics, Context};
use super::color_palette;

pub struct ShipMenu {
    bkgr_pos: glam::Vec2,
    bkgr_w: f32,
    bkgr_h: f32,
    buttons: Vec<button::Button>,
}


