use ggez::{
    graphics,
};
pub struct DrawingComponent {
    pub sprite: graphics::Image,
    pub image_size: (i32, i32),
    pub sprite_offset: (f32, f32),
    pub screen_pos: glam::Vec2,
    
}