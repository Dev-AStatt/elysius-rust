// *
// * This file is to hold all the functions associated with
// * The entity component system building functions, make make and 
// * other support functions to add items into the ECS systesm. 
// *
use ggez::graphics;
use rand::Rng;
use crate::entities::{
    Entities, 
    EntityIndex, 
    ObjectType,
};
use super::{
    draw_comp::DrawingComponent,
    pos_comp::PosComponent, 
    energy_comp::EnergyComponent, 
    orbit::OrbitalComponent
};

impl Entities {

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
        n_orbiting_ent_id: usize,
        n_orb_rad: i32,
    ) {
        let orb_comp = OrbitalComponent::new(
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
        n_orbiting_ent_id: usize,
        n_orb_rad: i32,
    ) {
        let orb_comp = OrbitalComponent::new(
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
    fn make_new_orbiting_body(
        self: &mut Self,
        b_type: ObjectType,
        entities_id: &mut Vec<EntityIndex>,
        sprite: graphics::Image,
        solar_system: i32,
        orbit_comp: Option<OrbitalComponent>,
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
        );
        self.draw_comp.push(new_draw_comp);

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






}
