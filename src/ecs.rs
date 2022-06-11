use ggez::{
    graphics::{self},
    Context,
};
use glam::{f32, i32, vec2};
// 0------------------Start of ECS Sstem---------------------------------------0
pub type EntityIndex = usize;

pub struct OrbitalComponent {
    pub orbiting_ent_id: usize,
    pub radius: i32,
    pub angle: f32,
    pub orbit_circle: graphics::Mesh,
    
}

pub struct DrawingComponent {
    pub sprite: graphics::Image,
    pub image_size: (i32, i32),
    pub sprite_offset: (f32, f32),
    
}

pub struct Entities {
    pub orbit_comp: Vec<Option<OrbitalComponent>>,
    pub draw_comp: Vec<DrawingComponent>,
    pub solar_pos_comp: Vec<(f32, f32)>,
    pub solar_system_id: Vec<i32>,
    //add a verify function to make sure all vectors stay the same length
}

impl Entities {
    //function will check that all vectors in the entities struct have the
    //same length and will return that length
    pub fn verify_vector_lengths(&self) -> usize {
        let vec_len = self.orbit_comp.len();
        if vec_len != self.draw_comp.len() {println!("Your ECS system has mismatched vectors");}
        if vec_len != self.solar_pos_comp.len() {println!("Your ECS system has mismatched vectors");}
        if vec_len != self.solar_system_id.len() {println!("Your ECS system has mismatched vectors");}   
        
        return vec_len;
    }
}



// 0--------------------End of ECS Sstem---------------------------------------0

//Function creates a new sun into the ECS system
pub fn make_new_sun(
    ents: &mut Entities,
    entities_id: &mut Vec<EntityIndex>,
    n_sprite: graphics::Image,
    n_sol_sys_id: i32,
    n_sol_pos: (f32 ,f32) 
    ) {

    //Verify that we are pushing the right numbers
    if ents.verify_vector_lengths() != entities_id.len() { return; }    
    //Drawing
    let new_draw_comp = DrawingComponent{
                        sprite: n_sprite,
                        image_size: (128,128),
                        sprite_offset: (64.0,64.0),    
                    };
    //Push everything to ents
    ents.draw_comp.push(new_draw_comp);
    ents.solar_pos_comp.push(n_sol_pos);
    ents.solar_system_id.push(n_sol_sys_id);
    //its a sun so no orbital info
    ents.orbit_comp.push(None);

    //Create a new entity ID
    entities_id.push(entities_id.len());
}
    
//Function creates a new planet into the ECS system
pub fn make_new_orbiting_body(
    ents: &mut Entities,
    entities_id: &mut Vec<EntityIndex>,
    current_ctx: &Context,
    n_sprite: graphics::Image,
    n_sol_sys_id: i32,
    n_orbiting_ent_id: usize,
    n_sol_pos: (f32,f32),
    n_orb_rad: i32,
    ) {

    //Verify that we are pushing the right numbers
    if ents.verify_vector_lengths() != entities_id.len() { return; }  
    
    //calc destination of orbital ring
    let dest = ents.solar_pos_comp[n_orbiting_ent_id];
    //get a new meshbuilder to make our circle
    let mb = &mut graphics::MeshBuilder::new();
    //get our new circle
    mb.circle(graphics::DrawMode::stroke(1.0),
        vec2(dest.0,dest.1),
        n_orb_rad as f32,
        2.0,
        graphics::Color::WHITE).expect("ecs new planet mesh error");

    let orbit_circle = graphics::Mesh::from_data(current_ctx, mb.build());

    //Drawing
    let new_draw_comp = DrawingComponent{
        sprite: n_sprite,
        image_size: (128,128),
        sprite_offset: (64.0,64.0), };
    //Orbit
    let new_orbit = OrbitalComponent {
        orbiting_ent_id: n_orbiting_ent_id,
        radius: n_orb_rad,
        angle: 25.0,
        orbit_circle,
    };
    //Push everything to ents
    ents.draw_comp.push(new_draw_comp);
    ents.orbit_comp.push(Some(new_orbit));
    ents.solar_pos_comp.push(n_sol_pos);
    ents.solar_system_id.push(n_sol_sys_id);
    //Create a new entity ID
    entities_id.push(entities_id.len());    
}


//Function will itterate through the active entities in solar system
//and update position
pub fn inc_orbital_body_pos(
    ents: &mut Entities,
    system_id: i32,
) {
    for i in 0..ents.solar_system_id.len() {
        if ents.solar_system_id[i] == system_id {
            match ents.orbit_comp[i] {
                None => {}
                Some(ref mut orb) => {
                    //increment angle
                    let adjustment = 0.1;   //This is what to mess around with to slow down
                    let mut new_angle = orb.angle + adjustment;
                    if new_angle > 360.0 {new_angle = new_angle - 360.0;}
                    orb.angle = new_angle;


                    //calculate new position
                    let unitx = (orb.angle * 3.14 / 180.0).sin();
                    let unity = (orb.angle * 3.14 / 180.0).cos();
                    let x = unitx * orb.radius as f32;
                    let y = unity * orb.radius as f32;
                    //give new position to ent
                    ents.solar_pos_comp[i].0 = x + ents.solar_pos_comp[orb.orbiting_ent_id].0;
                    ents.solar_pos_comp[i].1 = y + ents.solar_pos_comp[orb.orbiting_ent_id].1;
                    
                }
            }
            //update position of body
        }
    }
}