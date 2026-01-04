mod utils;

use lib_simulation as sim;
use rand::{prelude::*, rng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }
}
