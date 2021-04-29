use crate::ModelObjects::component::State;
use crate::DBMLib::lib;

#[derive(Clone)]
pub struct StatePair<'a> {
    pub states1: Vec<State<'a>>,
    pub states2: Vec<State<'a>>,
    pub zone: [i32; 1000],
    pub dimensions: u32,
}

impl<'b> StatePair<'b> {
    pub fn create<'a>(state1: Vec<State<'a>>, state2: Vec<State<'a>>) -> StatePair<'a> {
        return StatePair {
            states1: state1,
            states2: state2,
            zone: [0; 1000],
            dimensions: 0,
        };
    }

    pub fn get_states1(&self) -> &Vec<State> {
        &self.states1
    }

    pub fn get_states2(&self) -> &Vec<State> {
        &self.states2
    }

    pub fn get_mut_states1(&mut self) -> &mut Vec<State<'b>> {
        &mut self.states1
    }

    pub fn get_mut_states2(&mut self) -> &mut Vec<State<'b>> {
        &mut self.states2
    }

    pub fn get_dimensions(&self) -> u32 {
        self.dimensions.clone()
    }

    pub fn set_dimensions(&mut self, dim: u32) {
        self.dimensions = dim;
    }

    pub fn get_zone(&mut self) -> &mut [i32] {
        let dim = self.get_dimensions();
        let len = dim * dim;
        &mut self.zone[0..len as usize]
    }

    pub fn get_dbm_clone(&self) -> [i32; 1000] {
        return self.zone.clone();
    }

    pub fn set_dbm(&mut self, dbm: [i32; 1000]) {
        self.zone = dbm;
    }

    pub fn init_dbm(&mut self) {
        let mut dimensions = 1;
        for state in self.get_states1() {
            dimensions += state.get_dimensions();
        }

        for state in self.get_states2() {
            dimensions += state.get_dimensions();
        }
        self.dimensions = dimensions;
        lib::rs_dbm_zero(self.get_zone(), dimensions);
        lib::rs_dbm_up(self.get_zone(), dimensions);
    }
}