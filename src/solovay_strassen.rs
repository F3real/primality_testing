use ramp::{
    Int,
    RandomInt,
};
use rand;

use util::{
    jacobi_symbol,
    mod_exp,
};

// Returns True if probability that `n` is composite is less then 2**(`number_of_rounds`)
pub fn solovay_strassen(potential_prime: &Int, number_of_rounds: u32) -> bool {
    // check if number is even
    if potential_prime & 1 == 0  || potential_prime < &2 {
        return false;
    }
    let mut rng = rand::thread_rng();
    let n = (potential_prime - Int::one() ) / Int::from(2);
    for _ in 0..number_of_rounds 
    {
        let a = &rng.gen_int_range(&Int::from(2), potential_prime);
        //To solve situation in which jacobi is -1 and mod returns potential_prime - 1
        let jacobi = (potential_prime + jacobi_symbol(a, potential_prime).unwrap()) % potential_prime;
        if jacobi == Int::zero() || mod_exp(a, &n, potential_prime) != jacobi 
        {
            return false;
        }
    }

    true
}