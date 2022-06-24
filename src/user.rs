pub struct Player {
    pub energy: i32,
    energy_per_tick: i32,

}

impl Player {
    pub fn new() -> Self{
        Player {
            energy: 1000,
            energy_per_tick: 0,
        }
    }
} //End of Impl for Player