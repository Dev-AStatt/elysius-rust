use rand::Rng;

use ggez::{
    graphics::{self},
    Context,
};

use crate::ecs::orbit::OrbitalComponent;

use super::ecs::orbit;
use super::ecs::draw_comp::DrawingComponent;
use super::ecs::pos_comp::PosComponent;


#[derive(PartialEq, Clone, Copy)]
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

pub struct Entities {
    pub orbit_comp: Vec<Option<orbit::OrbitalComponent>>,
    pub draw_comp: Vec<DrawingComponent>,
    pub energy_comp: Vec<Option<EnergyComponent>>,
    //pub solar_pos_comp: Vec<glam::Vec2>,
    //pub solar_system_id: Vec<i32>,
    pub position_comp: Vec<PosComponent>,
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
            position_comp: Vec::new(),
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

    
    // pub fn get_orbit_final_pos(&self,
    //     ent_id: usize,
    //     scale: glam::Vec2,
    //     player_offset: glam::Vec2,
    // ) -> glam::Vec2 {
    //     let sprite_pos = glam::Vec2::new(
    //         self.solar_pos_comp[ent_id].x * scale.x,
    //         self.solar_pos_comp[ent_id].y * scale.y
    //     );
    //     let disp_adj = glam::Vec2::new(
    //         self.draw_comp[ent_id].sprite_offset.0 * scale.x,
    //         self.draw_comp[ent_id].sprite_offset.1 * scale.y
    //     );
    //     return sprite_pos - disp_adj + player_offset;
    // }

// 0-------------------------MAKE THINGS---------------------------------------0    

    //Function will itterate through the active entities in solar system
    //and update position
    pub fn inc_orbital_body_pos(
        self: &mut Self,
        system_id: i32,
    ) {
        // for i in 0..self.orbit_comp.len() {
        //     //if there is some orbital component at , then
        //     if let Some(ref mut orb) = self.orbit_comp[i] {
        //         //give new position to ent
        //         let pos_adj = orb.pos_adj();
        //         //self.solar_pos_comp[i] = pos_adj + self.solar_pos_comp[orb.orb_ent_id()];
        //     }             
        // }
        // }
        for i in 0..self.position_comp.len() {
            if let Some(ref mut orb) = self.orbit_comp[i] {
                let pos_orb_ent = self.position_comp[orb.orb_ent_id()].solar_pos();
                let or = orb.pos_adj();
                self.position_comp[i].set_solar_pos(or + pos_orb_ent );
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
        let orb_comp = orbit::OrbitalComponent::new(
            ctx,
            n_orb_rad,
            n_orbiting_ent_id,
        );
        self.make_new_orbiting_body(
            ObjectType::Planet,
            entities_id,
            n_sprite,
            n_sol_sys_id,
            Some(orb_comp)
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
        let orb_comp = orbit::OrbitalComponent::new(
            ctx,
            n_orb_rad,
            n_orbiting_ent_id,
        );

        self.make_new_orbiting_body(
            ObjectType::Ship,
            entities_id,
            n_sprite,
            n_sol_sys_id,
            Some(orb_comp)
        );
    }


    //Function creates a new planet into the ECS system
    pub fn make_new_orbiting_body(
        self: &mut Self,
        b_type: ObjectType,
        entities_id: &mut Vec<EntityIndex>,
        sprite: graphics::Image,
        solar_system: i32,
        orbit_comp: Option<orbit::OrbitalComponent>,
        ) {
        self.add_new_draw_comp(sprite);
        self.add_new_energy_comp(b_type);
        self.ent_type.push(b_type);
        self.add_new_pos_comp(solar_system, &orbit_comp);
        self.orbit_comp.push(orbit_comp);
        self.ent_name.push(self.get_new_name());
        //Create a new entity ID
        entities_id.push(entities_id.len());    
    }

    //Function will check if there is an orbital component, and then add
    //the position vector to ents
    fn add_new_pos_comp(
        self: &mut Self, 
        solar_system: i32,
        orb_comp: &Option<OrbitalComponent>,
    ) {
        let mut pos = glam::Vec2::new(0.0,0.0);
        //if there is some orbital component, that means the ent is orbiting
        //another body. Get that body position and offset it by the radius
        if let Some(orb) = orb_comp {
            pos = self.position_comp[orb.orb_ent_id()].solar_pos();
            pos.y += orb.rad() as f32; 
        } 
        self.position_comp.push(PosComponent::new(pos,solar_system));
    }

    //Function will add to the Vector energy component in the ECS system
    fn add_new_energy_comp(self: &mut Self, obj_type: ObjectType) {
        match obj_type {
            ObjectType::Sun => {
                self.energy_comp.push(Some(EnergyComponent::new()));
            } 
            ObjectType::Planet => {
                self.energy_comp.push(Some(EnergyComponent::new()));
            }
            ObjectType::Moon => {
                self.energy_comp.push(Some(EnergyComponent::new()));
            }
            ObjectType::Ship => {
                self.energy_comp.push(None);
            }
        }
    }

    fn add_new_draw_comp(self: &mut Self, sprite: graphics::Image) {
        //STEP 1 DRAWING COMPONENT 
        let sprite_width = sprite.width().try_into().unwrap();
        let sprite_height = sprite.height().try_into().unwrap();
        //Drawing
        let new_draw_comp = DrawingComponent::new(
            sprite,
            (sprite_width,sprite_height),
            glam::Vec2::new(sprite_width as f32 / 2.0, sprite_height as f32 / 2.0),
            glam::Vec2::new(0.0,0.0),
        );

        self.draw_comp.push(new_draw_comp);

    }


}


