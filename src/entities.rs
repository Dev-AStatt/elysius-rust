use rand::Rng;

use ggez::{
    graphics::{self},
    Context,
};

use super::ecs::orbit;

#[derive(PartialEq)]
pub enum ObjectType {
    Sun,
    Planet,
    Moon,
    Ship,
}

// 0------------------Start of ECS Sstem---------------------------------------0
pub type EntityIndex = usize;

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
    pub orbit_comp: Vec<Option<orbit::OrbitalComponent>>,
    pub draw_comp: Vec<DrawingComponent>,
    pub energy_comp: Vec<Option<EnergyComponent>>,
    pub solar_pos_comp: Vec<glam::Vec2>,
    pub solar_system_id: Vec<i32>,
    pub ent_name: Vec<String>,
    pub ent_type: Vec<ObjectType>,

  }

impl Entities {
    //Do NOT CREATE A NEW ENTITY FUNCTION UNLESS YOUR THE MAIN FN
    pub fn new() -> Entities {
        Entities{
            orbit_comp: Vec::new(),
            draw_comp: Vec::new(),
            energy_comp: Vec::new(),
            solar_pos_comp: Vec::new(),
            solar_system_id: Vec::new(),
            ent_name: Vec::new(),
            ent_type: Vec::new()
        }
    }


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
            self.solar_pos_comp[ent_id].x * scale.x,
            self.solar_pos_comp[ent_id].y * scale.y
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
                //if there is some orbital component at , then
                if let Some(ref mut orb) = self.orbit_comp[i] {
                    //give new position to ent
                    let pos_adj = orb.pos_adj();
                    self.solar_pos_comp[i] = pos_adj + self.solar_pos_comp[orb.orb_ent_id()];
                }             
                        
            }
        }




    }

    pub fn make_new_sun(
        self: &mut Self,
        entities_id: &mut Vec<EntityIndex>,
        n_sprite: graphics::Image,
        n_sol_sys_id: i32,
       
    ) {
        self.make_new_orbiting_body(
            ObjectType::Sun,
            entities_id,
            n_sprite,
            n_sol_sys_id,
            None
        );
    }

    pub fn make_new_planet(
        self: &mut Self,
        entities_id: &mut Vec<EntityIndex>,
        n_sprite: graphics::Image,
        n_sol_sys_id: i32,
        ctx: &Context,
        n_orbiting_ent_id: usize,
        n_orb_rad: i32,
    ) {
        let orbit_input = orbit::OptionalOrbitalInputs {
            ctx,
            orb_ent_id: n_orbiting_ent_id,
            orb_rad: n_orb_rad,
        };
        self.make_new_orbiting_body(
            ObjectType::Planet,
            entities_id,
            n_sprite,
            n_sol_sys_id,
            Some(orbit_input)
        );
    }

    pub fn make_new_ship(
        self: &mut Self,
        entities_id: &mut Vec<EntityIndex>,
        n_sprite: graphics::Image,
        n_sol_sys_id: i32,
        ctx: &Context,
        n_orbiting_ent_id: usize,
        n_orb_rad: i32,
    ) {
        let orbit_input = orbit::OptionalOrbitalInputs {
            ctx,
            orb_ent_id: n_orbiting_ent_id,
            orb_rad: n_orb_rad,
        };
        self.make_new_orbiting_body(
            ObjectType::Ship,
            entities_id,
            n_sprite,
            n_sol_sys_id,
            Some(orbit_input)
        );
    }


    //Function creates a new planet into the ECS system
    pub fn make_new_orbiting_body(
        self: &mut Self,
        b_type: ObjectType,
        entities_id: &mut Vec<EntityIndex>,
        n_sprite: graphics::Image,
        n_sol_sys_id: i32,
        orbit_inputs: Option<orbit::OptionalOrbitalInputs>,
        ) {

        //STEP 1 DRAWING COMPONENT 
        let sprite_width = n_sprite.width().try_into().unwrap();
        let sprite_height = n_sprite.height().try_into().unwrap();
        //Drawing
        let new_draw_comp = DrawingComponent{
            sprite: n_sprite,
            image_size: (sprite_width,sprite_height),
            sprite_offset: (sprite_width as f32 / 2.0, sprite_height as f32 / 2.0),
            screen_pos: glam::Vec2::new(0.0,0.0),
        };

        //STEP 2 OPTIONAL ORBITS
        //Opperations specific to the body type
        match b_type {
            ObjectType::Sun => {
                self.ent_type.push(ObjectType::Sun);
                self.energy_comp.push(Some(EnergyComponent::new()));
            } 
            ObjectType::Planet => {

                self.ent_type.push(ObjectType::Planet);
                self.energy_comp.push(Some(EnergyComponent::new()));
            }
            ObjectType::Moon => {

                self.ent_type.push(ObjectType::Moon);
                self.energy_comp.push(Some(EnergyComponent::new()));
            }
            ObjectType::Ship => {

                self.ent_type.push(ObjectType::Ship);
                self.energy_comp.push(None);
            }
        }    
        //if there is an orbital component passed in, we should make a 
        //orbital component for the ECS system.
        match orbit_inputs {
            Some(orb_inp) => {
                //solar position 
                let n_sol_pos = glam::Vec2::new(
                    self.solar_pos_comp[orb_inp.orb_ent_id].x,
                    self.solar_pos_comp[orb_inp.orb_ent_id].y + orb_inp.orb_rad as f32
                );
                self.solar_pos_comp.push(n_sol_pos);
                //Push to ECS orbital component the OrbitalComponent Struct
                //Returned by get_orbit and add as Some(). Sorry its so complicated
                self.orbit_comp.push(
                    //get orbital struct from get_orbit function
                    Some(orbit::OrbitalComponent::new(
                        orb_inp.ctx,
                        orb_inp.orb_rad,
                        orb_inp.orb_ent_id))
                );
            }
            None => {
                //Solar Position if no Orbit Comp
                self.solar_pos_comp.push(glam::Vec2::new(0.0,0.0));
                self.orbit_comp.push(None);
            }
        }
        //Push everything to ents
        self.draw_comp.push(new_draw_comp);
        
        self.solar_system_id.push(n_sol_sys_id);
        self.ent_name.push(self.get_new_name());
        //Create a new entity ID
        entities_id.push(entities_id.len());    
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