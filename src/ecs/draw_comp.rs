use ggez::graphics;

pub struct DrawingComponent {
    sprite: graphics::Image,
    image_size: (i32, i32),
    sprite_offset: glam::Vec2,
}

impl DrawingComponent {
    pub fn new(
        sprite: graphics::Image, 
        image_size: (i32, i32), 
        sprite_offset: glam::Vec2
    ) -> Self {
         Self { sprite, image_size, sprite_offset } 
    }

    //Getter Setters
    pub fn sprite(&self)        -> &graphics::Image {&self.sprite}
    pub fn image_size(&self)    -> (i32, i32)       {self.image_size}
    pub fn sprite_offset(&self) -> glam::Vec2       {self.sprite_offset}
}
