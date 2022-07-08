


pub fn point_in_square(tl: glam::Vec2, br: glam::Vec2, p: glam::Vec2) -> bool {

    if tl.x < p.x && p.x < br.x && tl.y < p.y && p.y < br.y {
        return true;
    } else {return false;}

}


