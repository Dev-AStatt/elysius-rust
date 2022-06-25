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


pub struct UIComponent {
    menu_type: MenuType,
    pos: (f32, f32),
    pub mesh: graphics::Mesh,
}

impl UIComponent {
    pub fn new_menu_orbit_body_info(
        ctx: &Context,    
        pos_init: (f32,f32)
    ) -> Self {
        let menu_type = MenuType::OrbitBodyInfo;
        
        let mesh = get_orbiting_body_info_mesh(ctx);


        UIComponent { 
            menu_type,
            pos: pos_init,
            mesh,
        }

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


//Function will build the orbiting body 
fn get_orbiting_body_info_mesh(
    ctx: &Context,
) -> graphics::Mesh {
    //Here is all the variable locations for Maps 

    let color_palette = ColorPalette::new();
    let mb = &mut graphics::MeshBuilder::new();
    mb.rounded_rectangle(
        graphics::DrawMode::fill(), 
        graphics::Rect::new(100.0,100.0,500.0,200.0),
        25.0, 
        color_palette.color_2,
    );
    
    
    
    return graphics::Mesh::from_data(ctx, mb.build());
}


