#![crate_name = "primality"]
#![crate_type = "lib"]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rand;
extern crate ramp;

pub use rabin_miller::{
    rabin_miller,
    rabin_miller_deterministic,
};

pub use baillie_psw::{
    baillie_psw
};

pub use solovay_strassen::{
    solovay_strassen
};

pub use util::{
    mod_exp,
    number_of_bits,
    number_of_digits,
    jacobi_symbol,
};

mod rabin_miller;
mod baillie_psw;
mod util;
mod solovay_strassen;