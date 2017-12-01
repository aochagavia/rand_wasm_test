#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate pcg_rand;

use pcg_rand::Pcg32Basic;
use rand::{Rng, SeedableRng};

use std::os::raw::c_double;
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<Pcg32Basic> = Mutex::new(Pcg32Basic::from_seed([42, 42]));
}

fn main() {}

#[no_mangle]
pub extern "C" fn get_rand() -> c_double {
    let rng = &mut RNG.lock().unwrap();
    rng.next_f64()
}
