use rand::Rng;

use ggez::{
    graphics,
    Context,
};

use crate::{
    ecs::orbit::OrbitalComponent,
    main_state::{game_state, event_system}
};

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


    pub fn update(
        self: &mut Self, 
        ids: &Vec<EntityIndex>,
        state: &game_state::GameState,
        events: &mut event_system::EventSystem,
    ) {
        //For everything in vect
        for i in 0..ids.len() {
            self.position_comp[i].update(state, self.draw_comp[i].sprite_offset());
        }
        //handle Events
        self.update_ent_events(events, ids);        

        if state.if_state_is(game_state::StateType::Running) {
            self.inc_orbital_body_pos();
        }
    }
    
  

    pub fn draw_objects(
        &self, 
        canvas: &mut graphics::Canvas,
        ctx: &Context,
        ids: &Vec<EntityIndex>,
        state: &game_state::GameState,
    ) {
        let mut all_tails: Vec<glam::Vec2> = Vec::new();
        //for each entity id in vect ent_ids
        ids.iter().for_each(|ref_ent| {
            let i = ref_ent.clone();  //Had to get id as a usize not a &usize 
            if self.position_comp[i].is_in_system(state.active_solar_system()) {
                //Then Draw 
                self.draw_sprite(canvas, i, state.scale());
                //Grab tails while were in here
                all_tails.append(&mut self.position_comp[i].sol_pos_history());
            }
        });
        //Draw Tails
        self.draw_orbit_tails(canvas, ctx, &state, all_tails);
    }

    

// 0-------------------------MAKE THINGS---------------------------------------0    

    //Function will itterate through the active entities in solar system
    //and update position
    fn inc_orbital_body_pos(
        self: &mut Self,
    ) {
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
        n_orbiting_ent_id: usize,
        n_orb_rad: i32,
    ) {
        let orb_comp = orbit::OrbitalComponent::new(
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
        let orb_comp = orbit::OrbitalComponent::new(
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






    //PRIVATE FUNCTIONS
    fn update_ent_events(
        self: &mut Self, 
        events: &mut event_system::EventSystem,
        ids: &Vec<EntityIndex>,
    ) {
        let new_events: Vec<event_system::Event> = events.get_events(event_system::EventType::InitShipTransfer);
        if new_events.len() == 0 {
            for i in 0..ids.len() {
                self.position_comp[i].set_in_transfer(false);
            } 
        }
         //for each event that is initiate ship transfer 
        new_events.into_iter().for_each(|e| {
            if let Some(ent_id) = e.generated_by() { //get ent_id from event
                //Do the event
                self.position_comp[ent_id].set_in_transfer(true);
                //if there is a target for the transfer
                if let Some(dest_id) = e.target() {
                    self.transfer_ship(ent_id, dest_id, events);
                }
            }
        });
   }

    fn transfer_ship(self: &mut Self, ent_id: usize, dest_id: usize, events: &mut event_system::EventSystem) {
        if ent_id == dest_id {return;}    //if dest is ent then cancel
        //need to handle the Option on ship OrbitalComponent
        if let Some(orb_comp) = &mut self.orbit_comp[ent_id] {
            //set new orbiting entity
            orb_comp.set_orbiting(dest_id);
            events.new_event_ez(event_system::EventType::ShipTransferComplete);
            self.position_comp[ent_id].set_in_transfer(false);
        }
    }
 
    fn draw_orbit_tails(
        &self, 
        canvas: &mut graphics::Canvas, 
        ctx: &Context,
        state: &game_state::GameState,
        tails: Vec<glam::Vec2>,
    ) {
        let mb = &mut graphics::MeshBuilder::new();
        tails.into_iter().for_each(|i| {
            mb.circle(
                graphics::DrawMode::fill(), 
                i, 
                10.0, 
                1.0, 
                graphics::Color::WHITE
            ).expect("Error in making tails");
        });
        let mesh = graphics::Mesh::from_data(ctx, mb.build());
    canvas.draw(&mesh, graphics::DrawParam::new());
         

    }

    fn draw_sprite(&self, canvas: &mut graphics::Canvas ,ent_id: usize, scale: glam::Vec2) {
        //Draw the sprite
        canvas.draw(self.draw_comp[ent_id].sprite(),
            graphics::DrawParam::new()
                .dest(self.position_comp[ent_id].screen_pos())
                .scale(scale)
        );
    }


    //Function creates a new planet into the ECS system
    fn make_new_orbiting_body(
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


