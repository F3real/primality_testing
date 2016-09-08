
extern crate ramp;
extern crate primality;

use ramp::Int;

use primality::{
    jacobi_symbol,
    number_of_bits,
    rabin_miller,
};


fn main(){
    let target = "9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999499999";

    println!("Number of digits: {}\n", target.len());
    println!("Number of bits(min): {}  Number of bits(max): {}\n", number_of_bits(target.len() as u32).0, number_of_bits(target.len() as u32).1);
    match rabin_miller(target, 64)
    {
        Ok(is_prime)=>{
                        if is_prime
                        {
                            println!("Number {} is prime",target);
                        }
                        else
                        {
                            println!("Number {} is not prime",target);
                        }
                    },
        Err(str)=> println!("{}",str),
    }
    println!("{}",jacobi_symbol(&Int::from(45455), &Int::from(54547)).unwrap() );
}