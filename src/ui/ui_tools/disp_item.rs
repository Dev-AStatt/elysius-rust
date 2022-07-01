
use ggez::{
    graphics::{self,Color},
    Context,
};
use super::color_palette;

pub enum BoxSize {
    Small,
    Large
}
//returns the value of width that size represents
impl BoxSize {
    fn get_size(&self) -> f32 {
        match self {
            BoxSize::Small => {return 150.0;}
            BoxSize::Large => {return 300.0;}
        }
    }
}

pub struct DisplayItem {
    pos: glam::Vec2,
    str_pos: glam::Vec2,
    disp_str: String,
    pub mesh: graphics::Mesh,
    icon: Option<graphics::Image>,
    text_color: Color
}

impl DisplayItem {
    pub fn new(
        pos: glam::Vec2, 
        size_type: BoxSize,
        ctx: &Context,
        disp_str: String,
        img: Option<graphics::Image>
    ) -> Self {
        //Box Size
        
        let disp_h = 50.0;
        let disp_w = size_type.get_size();
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
            graphics::Rect::new(pos.x, pos.y, disp_w, disp_h),
            15.0, 
            bkgr_color,
        ).expect("Rec Mesh Failed");
        let mesh = graphics::Mesh::from_data(ctx, mb.build());        
    
        return DisplayItem {
            pos,
            str_pos,
            disp_str,
            mesh,
            icon: img,
            text_color: Color::BLACK,
        };
    }
    

    pub fn draw_self(
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

