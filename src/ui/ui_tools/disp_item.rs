
use ggez::{
    graphics::{self,Color},
    Context,
};
use super::color_palette;
#[derive(PartialEq, Copy,Clone)]
pub enum BoxSize {
    Small,
    Large
}
//returns the value of width that size represents
impl BoxSize {
    pub fn get_width(&self) -> f32 {
        match self {
            BoxSize::Small => {return 150.0;}
            BoxSize::Large => {return 250.0;}
        }
    }
    pub fn get_hight(&self) -> f32 {
        match self {
            BoxSize::Small => {return 50.0;}
            BoxSize::Large => {return 50.0;}
        }
    }
    pub fn size(&self) -> glam::Vec2 {
        return glam::Vec2::new(self.get_width(), self.get_hight());
    }
}

#[derive(Clone)]
pub struct DisplayItem {
    pos: glam::Vec2,
    box_size: BoxSize,
    str_pos: glam::Vec2,
    disp_str: String,
    pub mesh: graphics::Mesh,
    icon: Option<graphics::Image>,
    text_color: Color
}

impl DisplayItem {
    pub fn new(
        pos: glam::Vec2, 
        box_size: BoxSize,
        ctx: &Context,
        disp_str: String,
        img: Option<graphics::Image>,
        bkgr_col: Option<Color>,
    ) -> Self {
        //figure out what color
        let mut bkgr_color = color_palette::ColorPalette::new().color_5;
        if let Some(col) = bkgr_col {bkgr_color = bkgr_col;}

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
            graphics::Rect::new(pos.x, pos.y, box_size.get_width(), box_size.get_hight()),
            15.0, 
            bkgr_color,
        ).expect("Rec Mesh Failed");
        let mesh = graphics::Mesh::from_data(ctx, mb.build());        
    
        return DisplayItem {
            pos,
            box_size,
            str_pos,
            disp_str,
            mesh,
            icon: img,
            text_color: Color::BLACK,
        };
    }
    
    //Getters and Setters
    pub fn hight(&self) -> f32 {return self.box_size.get_hight()} 
    pub fn width(&self) -> f32 {return self.box_size.get_width()} 
    pub fn box_size(&self) -> BoxSize {return self.box_size}
    pub fn rel_pos(&self) -> glam::Vec2 {return self.pos;}
    pub fn is_box_size(&self, b: BoxSize) -> bool {
        if b == self.box_size {return true;} 
        else {return false;}
    }


    pub fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        menu_pos: glam::Vec2,
    ) {
       
        let bkgr_color = color_palette::ColorPalette::new().color_5;
        self.draw_with_color(canvas, menu_pos, None);
    }

    pub fn draw_with_color(
        &self,
        canvas: &mut graphics::Canvas,
        menu_pos: glam::Vec2,
        bkgr_color: Option<Color>,
    ) {
        //Draw Back
        match bkgr_color {
            Some(col) => {
                canvas.draw(&self.mesh, graphics::DrawParam::new()
                    .dest(menu_pos)
                    .color(col)
                ); 
            }
            None => {
                canvas.draw(&self.mesh, graphics::DrawParam::new()
                    .dest(menu_pos)
                ); 
            }
        }


       let mut final_pos = menu_pos + self.str_pos;
       
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

