use ramp::{
    Int,
    RandomInt,
};
use std::str::FromStr;
use rand;

use util::mod_exp;

// Returns True if probability that `n` is composite is less then 4**(`number_of_rounds`)
pub fn rabin_miller(potential_prime: &Int, number_of_rounds: u32) -> bool {
    // check if number is even
    if potential_prime & 1 == 0  || potential_prime < &2 {
        return false;
    }
    // Check if number is to small enough to do deterministic check.
    if potential_prime < &Int::from_str("3317044064679887385961981").unwrap() {
        return rabin_miller_deterministic(potential_prime);
    }
    let mut rng = rand::thread_rng();
    // Split number in required form (2**s)*d.
    let (s, d): (u64, Int) = split_num(&(potential_prime - Int::one()));
    for _ in 0..number_of_rounds 
    {
        if is_rabin_miller_prime(&rng.gen_int_range(&Int::from(2), potential_prime),
                                 potential_prime,
                                 s,
                                 &d) {
            continue;
        }
        return false;
    }

    true
}

// Check if number is prime using precomputed witness values.
// Check is deterministic but can only be applied for values smaller then 3,317,044,064,679,887,385,961,981
pub fn rabin_miller_deterministic(potential_prime: &Int) -> bool {
    // Split number in required form (2**s)*d.
    let (s, d): (u64, Int) = split_num(&(potential_prime - Int::one()));
    if potential_prime < &Int::from(2047) {
        is_rabin_miller_prime(&Int::from(2), potential_prime, s, &d)
    } else if potential_prime < &Int::from(1_373_653) {
        rabin_miller_witness(potential_prime, &[2, 3], s, &d)
    } else if potential_prime < &Int::from(9_080_191) {
        rabin_miller_witness(potential_prime, &[31, 73], s, &d)
    } else if potential_prime < &Int::from(25_326_001) {
        rabin_miller_witness(potential_prime, &[2, 3, 5], s, &d)
    } else if potential_prime < &Int::from(3_215_031_751_i64) {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7], s, &d)
    } else if potential_prime < &Int::from(4_759_123_141_i64) {
        rabin_miller_witness(potential_prime, &[2, 7, 61], s, &d)
    } else if potential_prime < &Int::from(1_122_004_669_633_i64) {
        rabin_miller_witness(potential_prime, &[2, 13, 23, 1_662_803], s, &d)
    } else if potential_prime < &Int::from(2_152_302_898_747_i64) {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11], s, &d)
    } else if potential_prime < &Int::from(3_474_749_660_383_i64) {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11, 13], s, &d)
    } else if potential_prime < &Int::from(341_550_071_728_321_i64) {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11, 13, 17], s, &d)
    } else if potential_prime < &Int::from(3_825_123_056_546_413_051_i64) {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11, 13, 17, 19, 23], s, &d)
    } else if potential_prime < &Int::from_str("318665857834031151167461").unwrap() {
        rabin_miller_witness(potential_prime,
                             &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37],
                             s,
                             &d)
    } else if potential_prime < &Int::from_str("3317044064679887385961981").unwrap() {
        rabin_miller_witness(potential_prime,
                             &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41],
                             s,
                             &d)
    } else {
        // Return false if value is passed greater then 3,317,044,064,679,887,385,961,981
        // since it can't be deterministicly decided.
        false
    }
}

#[inline]
fn rabin_miller_witness(potential_prime: &Int, witnesses: &[i64], s: u64, d: &Int) -> bool {
    for witness in witnesses.iter() {
        if is_rabin_miller_prime(&Int::from(*witness), potential_prime, s, d) {
            continue;
        }
        return false;
    }
    true
}

#[inline]
pub fn is_rabin_miller_prime(num: &Int, potential_prime: &Int, s: u64, d: &Int) -> bool {
    if mod_exp(num, d, potential_prime) == 1 {
        true
    } else {
        for i in 0..s {
            let dd = d * Int::from(2).pow(i as usize);
            if mod_exp(num, &dd, potential_prime) == potential_prime - 1 {
                return true;
            }
        }
        false
    }
}

#[inline]
pub fn split_num(num: &Int) -> (u64, Int) {
    let mut d: Int = num.clone();
    let mut s: u64 = 0;
    while &d & 1 == 0 {
        d = &d >> 1;
        s += 1;
    }

    (s, d)
}

