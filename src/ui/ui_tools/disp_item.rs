
use ggez::{
    graphics::{self,Color},
    Context,
};
use super::color_palette;
#[derive(PartialEq, Clone)]
pub enum BoxSize {
    Small,
    Large
}
//returns the value of width that size represents
impl BoxSize {
    fn get_width(&self) -> f32 {
        match self {
            BoxSize::Small => {return 150.0;}
            BoxSize::Large => {return 300.0;}
        }
    }
    fn get_hight(&self) -> f32 {
        match self {
            BoxSize::Small => {return 50.0;}
            BoxSize::Large => {return 50.0;}
        }
    }
}

#[derive(Clone)]
pub struct DisplayItem {
    pos: glam::Vec2,
    size: BoxSize,
    str_pos: glam::Vec2,
    disp_str: String,
    pub mesh: graphics::Mesh,
    icon: Option<graphics::Image>,
    text_color: Color
}

impl DisplayItem {
    pub fn new(
        pos: glam::Vec2, 
        size: BoxSize,
        ctx: &Context,
        disp_str: String,
        img: Option<graphics::Image>
    ) -> Self {
        //Box Size
        
        let bkgr_color = color_palette::ColorPalette::new().color_5;

        let n_str_pos; 
        match img {
            Some(_) => {n_str_pos = (pos.x + 12.0, pos.y + 12.0);}
            None =>    {n_str_pos = (pos.x + 12.0 ,pos.y + 12.0);}
        }
        //Fix this line, its close enough for now
        let str_pos = glam::Vec2::new(n_str_pos.0, n_str_pos.1);
        
        //make mesh
        let mb = &mut graphics::MeshBuilder::new();
        mb.rounded_rectangle(
            graphics::DrawMode::fill(), 
            graphics::Rect::new(pos.x, pos.y, size.get_width(), size.get_hight()),
            15.0, 
            bkgr_color,
        ).expect("Rec Mesh Failed");
        let mesh = graphics::Mesh::from_data(ctx, mb.build());        
    
        return DisplayItem {
            pos,
            size,
            str_pos,
            disp_str,
            mesh,
            icon: img,
            text_color: Color::BLACK,
        };
    }
    
    //Getters and Setters
    pub fn hight(&self) -> f32 {return self.size.get_hight()} 
    pub fn width(&self) -> f32 {return self.size.get_width()} 
    pub fn is_box_size(&self, b: BoxSize) -> bool {
        if b == self.size {return true;} 
        else {return false;}
    }


    pub fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        pos_of_menu: glam::Vec2,
    ) {
        //Draw Back
        canvas.draw(&self.mesh, graphics::DrawParam::new().dest(pos_of_menu)); 
        let mut final_pos = pos_of_menu + self.str_pos;
       
        match self.icon {
            None => {}
            Some(ref s_icon) => {
                //Draw Icon and bump final position for text
                canvas.draw(s_icon, final_pos);
                final_pos.x += s_icon.width() as f32 + 10.0;
            }
        }
        canvas.draw(
            &graphics::Text::new(&self.disp_str), 
            graphics::DrawParam::new().dest(final_pos)
            .color(self.text_color)
            .scale(glam::Vec2::new(2.0,2.0)) 
        );        
    }
}

