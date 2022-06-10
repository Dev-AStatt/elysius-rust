use ggez::{
    event,
    graphics::{self, Color},
    Context, GameResult,
};
// 0------------------Start of ECS Sstem---------------------------------------0
pub type EntityIndex = usize;

pub struct OrbitalComponent {
    pub orbiting_ent_id: usize,
    pub radius: i32,
    pub angle: f32,
}

pub struct DrawingComponent {
    pub sprite: graphics::Image,
    pub image_size: (i32, i32),
    
}

pub struct Entities {
    pub orbit: Vec<Option<OrbitalComponent>>,
    pub draw_info: Vec<DrawingComponent>,
    pub solar_pos: Vec<(f32, f32)>,
    pub solar_system_id: Vec<i32>,
    //add a verify function to make sure all vectors stay the same length
}

impl Entities {
    //function will check that all vectors in the entities struct have the
    //same length and will return that length
    pub fn verify_vector_lengths(&self) -> usize {
        let vec_len = self.orbit.len();
        if vec_len != self.draw_info.len() {println!("Your ECS system has mismatched vectors");}
        if vec_len != self.solar_pos.len() {println!("Your ECS system has mismatched vectors");}
        if vec_len != self.solar_system_id.len() {println!("Your ECS system has mismatched vectors");}   
        
        return vec_len;
    }
}



// 0--------------------End of ECS Sstem---------------------------------------0

//Function creates a new sun into the ECS system
pub fn make_new_sun(
    entities: &mut Entities,
    entities_id: &mut Vec<EntityIndex>,
    n_sprite: graphics::Image,
    n_sol_sys_id: i32,
    n_sol_pos: (f32 ,f32) 
    ) {

    //Verify that we are pushing the right numbers
    if entities.verify_vector_lengths() != entities_id.len() { return; }    
    //Drawing
    let new_draw_comp = DrawingComponent{
                            sprite: n_sprite,
                            image_size: (128,128), };
    //Push everything to ents
    entities.draw_info.push(new_draw_comp);
    entities.solar_pos.push(n_sol_pos);
    entities.solar_system_id.push(n_sol_sys_id);
    //its a sun so no orbital info
    entities.orbit.push(None);

    //Create a new entity ID
    entities_id.push(entities_id.len());
}
    
//Function creates a new planet into the ECS system
pub fn make_new_planet(
    entities: &mut Entities,
    entities_id: &mut Vec<EntityIndex>,
    n_sprite: graphics::Image,
    n_sol_sys_id: i32,
    n_orbiting_ent_id: usize,
    n_sol_pos: (f32,f32),
    n_orb_rad: i32,
    ) {

    //Verify that we are pushing the right numbers
    if entities.verify_vector_lengths() != entities_id.len() { return; }    
    //Drawing
    let new_draw_comp = DrawingComponent{
        sprite: n_sprite,
        image_size: (128,128), };
    //Orbit
    let new_orbit = OrbitalComponent {
        orbiting_ent_id: n_orbiting_ent_id,
        radius: n_orb_rad,
        angle: 25.0,
    };
    //Push everything to ents
    entities.draw_info.push(new_draw_comp);
    entities.orbit.push(Some(new_orbit));
    entities.solar_pos.push(n_sol_pos);
    entities.solar_system_id.push(n_sol_sys_id);
    //Create a new entity ID
    entities_id.push(entities_id.len());    
}
//Function will itterate through the active entities in solar system
//and update position
pub fn update_orbital_body_positions(
    ents: &mut Entities,
    ents_id: &mut Vec<EntityIndex>,
    system_id: i32,
) {
    for i in 0..ents.solar_system_id.len() {
        if ents.solar_system_id[i] == system_id {
            match ents.orbit[i] {
                None => {}
                Some(ref mut orb) => {
                    //increment angle
                    let adjustment = 0.1;
                    let mut new_angle = orb.angle + adjustment;
                    if new_angle > 360.0 {new_angle = new_angle - 360.0;}
                    orb.angle = new_angle;


                    //calculate new position
                    let unitx = (orb.angle * 3.14 / 180.0).sin();
                    let unity = (orb.angle * 3.14 / 180.0).cos();
                    let x = unitx * orb.radius as f32;
                    let y = unity * orb.radius as f32;
                    //give new position to ent
                    ents.solar_pos[i].0 = x + ents.solar_pos[orb.orbiting_ent_id].0;
                    ents.solar_pos[i].1 = y + ents.solar_pos[orb.orbiting_ent_id].1;
                    
                }
            }
            //update position of body
        }
    }
}