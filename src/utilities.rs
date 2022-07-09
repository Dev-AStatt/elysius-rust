use ggez::{Context, graphics};




pub fn point_in_square(tl: glam::Vec2, br: glam::Vec2, p: glam::Vec2) -> bool {

    if tl.x < p.x && p.x < br.x && tl.y < p.y && p.y < br.y {
        return true;
    } else {return false;}

}

pub fn point_in_circle(point: &(f32,f32), center: (f32, f32), r: f32) -> bool {
    let dx = (point.0-center.0).abs();
    let dy = (point.1-center.1).abs();
    //test points
    if dx > r as f32 {return false;}
    if dy > r as f32 {return false;}
    if dx + dx <= r as f32 {return true;}
    if (dx*dx) + (dy*dy) <= (r * r) as f32 {return true;}
    else {return false;}
}

pub fn point_in_circle_vec2(point: &glam::Vec2, center: glam::Vec2, r: f32) -> bool {
    return point_in_circle(
        &(point.x, point.y),
        (center.x,center.y), 
        r
    );
}


//Function will take a path to the sprite that wants to load and return either
//that sprite or a red cube of 5x5 if there was an error
pub fn sprite_get(ctx: &Context, path: &str) -> graphics::Image {
    match graphics::Image::from_path(ctx, path, true) {
        Ok(it) => return it,
        Err(err) => {
            println!("Pub fn sprite_get: Error on loading path: {}", path);
            println!("Error: {}", err);
            return graphics::Image::from_solid(ctx, 5, graphics::Color::RED);
        }
    };
}
