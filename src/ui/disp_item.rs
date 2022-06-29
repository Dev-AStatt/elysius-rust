
use ggez::{
    graphics::{self,Color},
    Context,
};
use super::color_palette;

pub struct DisplayItem {
    pos: glam::Vec2,
    str_pos: glam::Vec2,
    w: f32,
    h: f32,
    pub mesh: graphics::Mesh,
    icon: Option<graphics::Image>,
    focus: bool,
    focus_col_on: Color, 
    focus_col_off: Color, 
}

impl DisplayItem {
    pub fn new(n_pos: (f32, f32), ctx: &Context, img: Option<graphics::Image>) -> Self {
        let disp_w = 150.0;
        let disp_h = 50.0;
        let pos = glam::Vec2::new(n_pos.0, n_pos.1);

        let n_str_pos; 
        match img {
            Some(_) => {n_str_pos = (pos.x + 35.0, pos.y + 2.0);}
            None =>    {n_str_pos = (pos.x + 2.0, pos.y + 2.0); }
        }
        let str_pos = glam::Vec2::new(n_str_pos.0, n_str_pos.1 + 10.0);
        let col_palette = color_palette::ColorPalette::new();
        
        //make mesh
        let mb = &mut graphics::MeshBuilder::new();
        mb.rounded_rectangle(
            graphics::DrawMode::fill(), 
            graphics::Rect::new(pos.x, pos.y, disp_w, disp_h),
            15.0, 
            col_palette.color_3,
        ).expect("Rec Mesh Failed");
        let mesh = graphics::Mesh::from_data(ctx, mb.build());        
    
        return DisplayItem {
            pos,
            str_pos,
            w: 50.0,
            h: 36.0,
            mesh,
            icon: img,
            focus: false,
            focus_col_on: col_palette.color_5,
            focus_col_off: col_palette.color_4,
        };
    }

    pub fn draw_self(
        &self,
        canvas: &mut graphics::Canvas,
        pos_of_menu: glam::Vec2,
        disp_str: String,
    ) {
        //Draw Back
        canvas.draw(&self.mesh, graphics::DrawParam::new().dest(pos_of_menu)); 
        let mut final_pos = pos_of_menu + self.str_pos;
       
        //seperate if the display icon has an icon to display or not
        match self.icon {
            None => {
            }
            Some(ref s_icon) => {
                final_pos.x -= s_icon.width() as f32;
                canvas.draw(
                    s_icon,
                    final_pos
                );
                final_pos.x += s_icon.width() as f32 + 2.0;
            }
        }
        canvas.draw(
            &graphics::Text::new(&disp_str), 
            graphics::DrawParam::new().dest(final_pos)
            .color(Color::BLACK)
            .scale(glam::Vec2::new(2.0,2.0)) 
        );        
       
    }
}
