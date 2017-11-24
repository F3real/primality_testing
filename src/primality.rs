#![crate_name = "primality"]
#![crate_type = "lib"]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate ramp;
extern crate rand;

use std::str::FromStr;
use ramp::{RandomInt, Int};


/*Returns True if probability that `n` is composite is less then 4**(`required_probability`)*/
pub fn rabin_miller(potential_prime: &Int, required_probability: u32) -> bool
{
   /*Check if number is to small enough to do deterministic check.*/
   if potential_prime  < &Int::from_str("3317044064679887385961981").unwrap()
   {
       return rabin_miller_deterministic(potential_prime);
   }
   /*check if number is even*/
   if potential_prime & 1 == 0
   {
       return false;
   }
   let mut rng = rand::thread_rng();
   let mut probability_is_composite: u32 = 0;
   /*Split number in required form (2**s)*d.*/
   let (s, d): (u64, Int) = split_num( &(potential_prime - Int::one()) );
   while probability_is_composite < required_probability
   {
       if is_rabin_miller_prime( &rng.gen_int_range( &Int::from(2), potential_prime), potential_prime, s, &d)
       {
           probability_is_composite += 1;
           continue;
       }
       return false;
   }

   true
}

/* Returns probability that number is composite given bit size of number and number of 
rounds. Returns correct results in case:
  bit size >=21
  3 <= number_of_rounds <= bit_size / 9
  From  Average case error estimates for the strong probable prime test by Damgard-Landrock-Pomerance*/
pub fn get_rabin_miller_probability(bit_size: u32, number_of_rounds: u32) -> Result<f64, String>
{
   let k = f64::from(bit_size);
   let t = f64::from(number_of_rounds);
   if bit_size >= 21 && number_of_rounds >= 3 && number_of_rounds <= bit_size/9
   {
       Ok(k.powf(1.5) * 2f64.powf(t) * t.powf(-1.5) * 4f64.powf(2f64 - (k*t).sqrt()))
   }
   else
   {
       Err(String::from("Requirements are not met."))
   }
}

/*Check if number is prime using precomputed witness values.
Check is deterministic but can only be applied for values smaller then 3,317,044,064,679,887,385,961,981*/
pub fn rabin_miller_deterministic(potential_prime: &Int) -> bool
{
    /*Split number in required form (2**s)*d.*/
    let (s, d): (u64, Int) = split_num( &(potential_prime - Int::one()) );
    if potential_prime < &Int::from(2047)
    {
        is_rabin_miller_prime( &Int::from(2), potential_prime, s, &d)
    }
    else if potential_prime < &Int::from(1_373_653)
    {
        rabin_miller_witness(potential_prime, &[2, 3], s, &d)
    }
    else if potential_prime < &Int::from(9_080_191)
    {
        rabin_miller_witness(potential_prime, &[31, 73], s, &d)
    }
    else if potential_prime < &Int::from(25_326_001)
    {
        rabin_miller_witness(potential_prime, &[2, 3, 5], s, &d)
    }
    else if potential_prime < &Int::from(3_215_031_751_i64)
    {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7], s, &d)
    }
    else if potential_prime < &Int::from(4_759_123_141_i64)
    {
        rabin_miller_witness(potential_prime, &[2, 7, 61], s, &d)
    }
    else if potential_prime < &Int::from(1_122_004_669_633_i64)
    {
        rabin_miller_witness(potential_prime, &[2, 13, 23, 1_662_803], s, &d)
    }
    else if potential_prime < &Int::from(2_152_302_898_747_i64)
    {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11], s, &d)
    }
    else if potential_prime < &Int::from(341_550_071_728_321_i64)
    {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11, 13], s, &d)
    }
    else if potential_prime < &Int::from(341_550_071_728_321_i64)
    {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11, 13, 17], s, &d)
    }
    else if potential_prime < &Int::from(3_825_123_056_546_413_051_i64)
    {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11, 13, 17, 19, 23], s, &d)
    }
    else if potential_prime < &Int::from_str("318665857834031151167461").unwrap()
    {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37], s, &d)
    }
    else if potential_prime < &Int::from_str("3317044064679887385961981").unwrap()
    {
        rabin_miller_witness(potential_prime, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41], s, &d)
    }
    else
    {
        /* Return false if value is passed greater then 3,317,044,064,679,887,385,961,981
          since it can't be deterministicly decided.*/
        false
    }
}


#[inline]
fn rabin_miller_witness(potential_prime: &Int, witnesses: &[i64], s: u64, d: &Int) -> bool
{
    for witness in witnesses.iter()
    {
        if is_rabin_miller_prime( &Int::from(*witness), potential_prime, s, d)
        {
            continue;
        }
        return false;
    }
    true
}


fn is_rabin_miller_prime(num: &Int, potential_prime: &Int, s: u64, d: &Int) -> bool
{
   if mod_exp( num, d, potential_prime) == 1
   {
       true
   }
   else
   {
       for i in 0..s
       {
           let dd =  d * Int::from(2).pow(i as usize);
           if mod_exp( num, &dd, potential_prime) ==  potential_prime -1
           {
               return true;
           }
       }
       false
   }
}


#[inline]
fn split_num(num: &Int) -> (u64, Int) {
 let mut d: Int = num.clone();
 let mut s: u64 = 0;
 while &d & 1 == 0
 {
   d = &d >> 1;
   s += 1;
 }

 (s, d)
}


fn mod_exp(base: &Int, exponent: &Int, modulus: &Int) -> Int {

   let mut base = base.clone() % modulus;
   let mut exponent = exponent.clone();
   let mut result = Int::one();

   while exponent > 0
   {
       if &exponent & 1 == 1
       {
           result = (&base * result) % modulus;
       }

       base = (&base * &base.clone()) % modulus;
       exponent = &exponent >> 1;
   }

   result
}


/*Returns (min bits, max bits) touple for number with given number of digits*/
pub fn number_of_bits(number_of_digits:u32) -> (u32, u32)
{
   let log10_b2: f64 = 3.321_928_094_89_f64;
   (
       ( log10_b2 * ( f64::from(number_of_digits) - 1f64)).ceil() as u32,

       ( f64::from(number_of_digits) * log10_b2).ceil() as u32
   )
}

/*Returns number of digits in a number*/
pub fn number_of_digits(test_number: &Int) -> u32
{ 
  let mut digit_number = 0;
  let mut d = test_number.clone();
  while d != 0
  {
      d = &d / 10;
      digit_number += 1;
  }
  digit_number
}

/*Implementation of Baillie–PSW primality test.*/
pub fn baillie_psw(potential_prime: &Int) -> bool
{
   //check if number is even
   if potential_prime & 1 == 0
   {
       return false;
   }
   /*Firstly perform rabin-miller test with base 2.*/
   let (s, d): (u64, Int) = split_num(potential_prime);
   if !is_rabin_miller_prime( &Int::from(2), potential_prime, s, &d)
   {
       return false;
   }
   /*Find the first D in the sequence 5, -7, 9, -11, 13, -15, ... for which the Jacobi symbol (D/n) is −1..*/
   /*TODO*/
   true
}


/*LR jacobi algorithm.*/
pub fn jacobi_symbol( d: &Int, n: &Int) -> Result<Int, String>
{
   let mut result: Int = Int::one();
   let mut d = d.clone();
   let mut n = n.clone();
   if &n & 1 == 0
   {
       return Err("Jacobi symbol are not defined for even n.".to_string());
   }
   if d < 0
   {
       d = - d;
       if &n % 4 == 3
       {
           result = - result;
       }
   }
   while d != 0
   {
       while &d & 1 == 0
       {
           d = &d >> 1;
           match i32::from( & ( &n % 8) )
           {
               3 | 5=> result = - result,
               _ => {}
           }
       }
       d = &d ^ &n;
       n = &d ^ &n;
       d = &d ^ &n;
       if &d % 4 == 3 && &n % 4 == 3
       {
           result = - result;
       }
       d = &d % &n;
   }
   if n == 1
   {
       Ok(result)
   }
   else
   {
       Ok(Int::zero())
   }
}
