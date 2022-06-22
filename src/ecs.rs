use rand::Rng;

use ggez::{
    graphics::{self},
    Context,
};
use glam::{f32, i32, vec2};
use crate::globs;
// 0------------------Start of ECS Sstem---------------------------------------0
pub type EntityIndex = usize;



pub struct OrbitalComponent {
    pub orbiting_ent_id: usize,
    pub radius: i32,
    pub angle: f32,
    pub orbit_circle: graphics::Mesh,
    
}

pub struct EnergyComponent {
    //units of energy
    pub fossil: i32,
    pub radioactive: i32,
}

impl EnergyComponent {
    pub fn new() -> Self {
        let fossil = 100;
        let radioactive = 50;
        EnergyComponent {
            fossil,
            radioactive, 
        }
    }
}

pub struct DrawingComponent {
    pub sprite: graphics::Image,
    pub image_size: (i32, i32),
    pub sprite_offset: (f32, f32),
    pub screen_pos: glam::Vec2,
    
}

pub struct Entities {
    pub orbit_comp: Vec<Option<OrbitalComponent>>,
    pub draw_comp: Vec<DrawingComponent>,
    pub energy_comp: Vec<Option<EnergyComponent>>,
    pub solar_pos_comp: Vec<(f32, f32)>,
    pub solar_system_id: Vec<i32>,
    pub ent_name: Vec<String>,

   
   

}

impl Entities {
 
    fn get_new_name(&self) -> String {
        let mut rng = rand::thread_rng();
        let names = vec![
            "Lodania Minor",
            "Paumi",
            "Padikar 230",
            "Roshar",
            "Dune",
            "Arrakis",
            "Helios",
            "Dimos",
            "Perseus",
            "Ares",
        ];
        let i = rng.gen_range(0..names.len());
        return names[i].to_string();
    }

    
    pub fn get_orbit_final_pos(&self,
        ent_id: usize,
        scale: glam::Vec2,
        player_offset: glam::Vec2,
    ) -> glam::Vec2 {
        let sprite_pos = glam::Vec2::new(
            self.solar_pos_comp[ent_id].0 * scale.x,
            self.solar_pos_comp[ent_id].1 * scale.y
        );
        let disp_adj = glam::Vec2::new(
            self.draw_comp[ent_id].sprite_offset.0 * scale.x,
            self.draw_comp[ent_id].sprite_offset.1 * scale.y
        );
        return sprite_pos - disp_adj + player_offset;
    }

// 0-------------------------MAKE THINGS---------------------------------------0    

    //Function will itterate through the active entities in solar system
    //and update position
    pub fn inc_orbital_body_pos(
        self: &mut Self,
        system_id: i32,
    ) {
        for i in 0..self.solar_system_id.len() {
            if self.solar_system_id[i] == system_id {
                match self.orbit_comp[i] {
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
                        self.solar_pos_comp[i].0 = x + self.solar_pos_comp[orb.orbiting_ent_id].0;
                        self.solar_pos_comp[i].1 = y + self.solar_pos_comp[orb.orbiting_ent_id].1;
                        
                    }
                }
                //update position of body
            }
        }
    }


    //Function creates a new planet into the ECS system
    pub fn make_new_orbiting_body(
        self: &mut Self,
        entities_id: &mut Vec<EntityIndex>,
        current_ctx: &Context,
        n_sprite: graphics::Image,
        n_sol_sys_id: i32,
        n_orbiting_ent_id: usize,
        n_orb_rad: i32,
        ) {

        //Verify that we are pushing the right numbers
        if self.verify_vector_lengths() != entities_id.len() { return; }  
        
        //get a new meshbuilder to make our circle
        let mb = &mut graphics::MeshBuilder::new();
        //get our new circle
        mb.circle(graphics::DrawMode::stroke(1.0),
            vec2(0.0,0.0), //dest.0,dest.1
            n_orb_rad as f32,
            2.0,
            graphics::Color::WHITE).expect("ecs new planet mesh error");

        let orbit_circle = graphics::Mesh::from_data(current_ctx, mb.build());

        let sprite_width = n_sprite.width().try_into().unwrap();
        let sprite_height = n_sprite.height().try_into().unwrap();
        //Drawing
        let new_draw_comp = DrawingComponent{
            sprite: n_sprite,
            image_size: (sprite_width,sprite_height),
            sprite_offset: (sprite_width as f32 / 2.0, sprite_height as f32 / 2.0),
            screen_pos: glam::Vec2::new(0.0,0.0),
        };
        //Orbit
        let new_orbit = OrbitalComponent {
            orbiting_ent_id: n_orbiting_ent_id,
            radius: n_orb_rad,
            angle: 25.0,
            orbit_circle,
        };
        let n_sol_pos = (
            self.solar_pos_comp[n_orbiting_ent_id].0,
            self.solar_pos_comp[n_orbiting_ent_id].1 + n_orb_rad as f32
        );

        //Push everything to ents
        self.draw_comp.push(new_draw_comp);
        self.orbit_comp.push(Some(new_orbit));
        self.energy_comp.push(Some(EnergyComponent::new()));
        self.solar_pos_comp.push(n_sol_pos);
        self.solar_system_id.push(n_sol_sys_id);
        self.ent_name.push(self.get_new_name());
        //Create a new entity ID
        entities_id.push(entities_id.len());    
    }

    //Function creates a new sun into the ECS system
    pub fn make_new_sun(
        self: &mut Self,
        entities_id: &mut Vec<EntityIndex>,
        n_sprite: graphics::Image,
        n_sol_sys_id: i32,
        n_sol_pos: (f32 ,f32) 
        ) {

        //Verify that we are pushing the right numbers
        if self.verify_vector_lengths() != entities_id.len() { return; }    
        //Drawing
        let new_draw_comp = DrawingComponent{
                            sprite: n_sprite,
                            image_size: (128,128),
                            sprite_offset: (64.0,64.0), 
                            screen_pos: glam::Vec2::new(0.0,0.0),   
                        };
        let new_energy_comp = EnergyComponent::new();
        //Push everything to ents
        self.draw_comp.push(new_draw_comp);
        self.solar_pos_comp.push(n_sol_pos);
        self.solar_system_id.push(n_sol_sys_id);
        //its a sun so no orbital info
        self.orbit_comp.push(None);
        self.energy_comp.push(Some(new_energy_comp));
        self.ent_name.push(self.get_new_name());

        //Create a new entity ID
        entities_id.push(entities_id.len());
    }

    //function will check that all vectors in the entities struct have the
    //same length and will return that length
    fn verify_vector_lengths(&self) -> usize {
        let vec_len = self.orbit_comp.len();
        if vec_len != self.draw_comp.len() {println!("Your ECS system has mismatched vectors");}
        if vec_len != self.solar_pos_comp.len() {println!("Your ECS system has mismatched vectors");}
        if vec_len != self.solar_system_id.len() {println!("Your ECS system has mismatched vectors");}   
        
        return vec_len;
    }
}



// 0--------------------End of ECS System---------------------------------------0

pub fn point_in_object(point: &(f32,f32), center: (f32, f32), r: f32) -> bool {
    let dx = (point.0-center.0).abs();
    let dy = (point.1-center.1).abs();
    //test points
    if dx > r as f32 {return false;}
    if dy > r as f32 {return false;}
    if dx + dx <= r as f32 {return true;}
    if (dx*dx) + (dy*dy) <= (r * r) as f32 {return true;}
    else {return false;}
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