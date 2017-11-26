use ramp::Int;

use util::jacobi_symbol;
use rabin_miller::{
    is_rabin_miller_prime,
    split_num,
};

// Implementation of Baillie–PSW primality test.
pub fn baillie_psw(potential_prime: &Int) -> bool {
    // check if number is even
    if potential_prime & 1 == 0 {
        return false;
    }
    // Firstly perform rabin-miller test with base 2.
    let (s, d): (u64, Int) = split_num(potential_prime);
    if !is_rabin_miller_prime(&Int::from(2), potential_prime, s, &d) {
        return false;
    }
    // Find the first D in the sequence 5, -7, 9, -11, 13, -15, ... for which the Jacobi symbol (D/n) is −1..
    let mut d_jacobi = Int::from(5);
    let mut two = Int::from(2);
    let m_one = Int::from(-1);
    while jacobi_symbol(&d_jacobi, potential_prime).unwrap() != m_one
    {
        two = -two;
        d_jacobi = &two - d_jacobi;
    }
    //TODO lucas test
    true
}