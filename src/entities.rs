
use ggez::{
    graphics,
    Context,
};

use crate::main_state::{game_state, event_system};

use super::ecs::{
    orbit::OrbitalComponent,
    draw_comp::DrawingComponent,
    pos_comp::PosComponent,
    tail::Tail,
    energy_comp::EnergyComponent,
};

#[derive(PartialEq, Clone, Copy)]
pub enum ObjectType {
    Sun,
    Planet,
    Moon,
    Ship,
}

// 0------------------Start of ECS Sstem---------------------------------------0
pub type EntityIndex = usize;

pub struct Entities {
    pub orbit_comp:     Vec<Option<OrbitalComponent>>,
    pub draw_comp:      Vec<DrawingComponent>,
    pub energy_comp:    Vec<Option<EnergyComponent>>,
    pub position_comp:  Vec<PosComponent>,
    pub ent_name:       Vec<String>,
    pub ent_type:       Vec<ObjectType>,
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


//  0---------------------------------------------------0
//  |                   UIPDATE FUNCTIONS               |
//  0---------------------------------------------------0

    pub fn update(
        self: &mut Self, 
        ids: &Vec<EntityIndex>,
        state: &game_state::GameState,
        events: &mut event_system::EventSystem,
        
    ) {
        self.update_ent_events(events, ids);        
        if state.if_state_is(game_state::StateType::Running) {
            self.update_positions(ids, state);
        }
    }

    fn update_positions(
        self: &mut Self, 
        ids: &Vec<EntityIndex>,
        state: &game_state::GameState,
    ) {
        //For everything in vect
        (0..ids.len()).for_each(|i| {
            //get orbiting component 
            let orb_pos: Option<glam::Vec2>;
            let mut orb_ent_id: usize = 0;
            let mut new_pos: glam::Vec2 = glam::Vec2::new(0.0,0.0); 
            match self.orbit_comp[i] {
                Some(ref mut orb) => {
                    let temp_orb_pos = self.position_comp[orb.orb_ent_id()].solar_pos();
                    orb_ent_id = orb.orb_ent_id();
                    new_pos =  orb.pos_adj() + temp_orb_pos;
                    orb_pos = Some(temp_orb_pos);
                }
                None => {orb_pos = None;}
            }
            self.position_comp[i].update(
                state, 
                self.draw_comp[i].sprite_offset(),
                orb_pos,
                orb_ent_id,
                new_pos,
            );
        });
    }

    fn update_ent_events(
        self: &mut Self, 
        events: &mut event_system::EventSystem,
        ids: &Vec<EntityIndex>,
    ) {
        //Get events
        let new_events: Vec<event_system::Event> = 
            events.get_events(event_system::EventType::InitShipTransfer);
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

    fn transfer_ship(
        self: &mut Self, 
        ent_id: usize, 
        dest_id: usize, 
        events: &mut event_system::EventSystem
    ) {
        if ent_id == dest_id {return;}    //if dest is ent then cancel
        //need to handle the Option on ship OrbitalComponent
        if let Some(orb_comp) = &mut self.orbit_comp[ent_id] {
            //set new orbiting entity
            orb_comp.set_orbiting(dest_id);
            events.new_event_ez(event_system::EventType::ShipTransferComplete);
            self.position_comp[ent_id].set_in_transfer(false);
        }
    }
 


//  0---------------------------------------------------0
//  |                   DRAW FUNCTIONS                  |
//  0---------------------------------------------------0
    pub fn draw_objects(
        &self, 
        canvas: &mut graphics::Canvas,
        ctx: &Context,
        ids: &Vec<EntityIndex>,
        state: &game_state::GameState,
    ) {
        self.draw_tails(canvas, ctx, ids, state);
        self.draw_sprites(canvas, ids, state);
   }

    fn draw_tails(
        &self, 
        canvas: &mut graphics::Canvas,
        ctx: &Context,
        ids: &Vec<EntityIndex>,
        state: &game_state::GameState,
    ) {
        let mut all_tails: Vec<Tail> = Vec::new();
        //for each entity id in vect ent_ids
        ids.iter().for_each(|ref_ent| {
            let i = ref_ent.clone();  //Had to get id as a usize not a &usize 
            if self.position_comp[i].is_in_system(state.active_solar_system()) {
                    all_tails.append(&mut self.position_comp[i].tails());
                }
        });
        //Draw Tails
        let mesh = self.build_tail_mesh(ctx, state, all_tails);
        canvas.draw(
            &mesh, 
            graphics::DrawParam::new()
        );

    }

    fn build_tail_mesh(
        &self, 
        ctx: &Context,
        state: &game_state::GameState,
        tails: Vec<Tail>,
    ) -> graphics::Mesh {
        let mb = &mut graphics::MeshBuilder::new();
        //Loop for each tail
        tails.into_iter().for_each(|t| {
            //get position of orbiting entity
            let orbit_pos = self.position_comp[t.orbit_id()].solar_pos();
            let mut color = graphics::Color::WHITE;
            if t.hilighted() {color = graphics::Color::GREEN;}
            //Make the circle mesh
            mb.circle(
                graphics::DrawMode::fill(), 
                t.calc_final_tail_pos(state, orbit_pos), 
                3.0, 
                1.0,   
                color
            ).expect("Error in making tails");
        });
        //mash all the circles together
        return graphics::Mesh::from_data(ctx, mb.build());
    }

    fn draw_sprites(
        &self,
        canvas: &mut graphics::Canvas,
        ids: &Vec<EntityIndex>,
        state: &game_state::GameState,
    ) {        
        //for each entity id in vect ent_ids
        ids.iter().for_each(|ref_ent| {
            let i = ref_ent.clone();  //Had to get id as a usize not a &usize 
            if self.position_comp[i].is_in_system(state.active_solar_system()) {
                //Then Draw 
                self.draw_single_sprite(canvas, i, state.scale());
            }
        });
    }  

    fn draw_single_sprite(&self, canvas: &mut graphics::Canvas ,ent_id: usize, scale: glam::Vec2) {
        //Draw the sprite
        canvas.draw(self.draw_comp[ent_id].sprite(),
            graphics::DrawParam::new()
                .dest(self.position_comp[ent_id].screen_pos())
                .scale(scale)
        );
    }
}


