use ramp::Int;


pub fn mod_exp(base: &Int, exponent: &Int, modulus: &Int) -> Int {

    let mut base = base.clone() % modulus;
    let mut exponent = exponent.clone();
    let mut result = Int::one();

    while exponent > 0 {
        if &exponent & 1 == 1 {
            result = (&base * result) % modulus;
        }

        base = (&base * &base.clone()) % modulus;
        exponent = &exponent >> 1;
    }

    result
}

// Returns (min bits, max bits) touple for number with given number of digits
pub fn number_of_bits(number_of_digits: u32) -> (u32, u32) {
    let log10_b2: f64 = 3.321_928_094_89_f64;
    ((log10_b2 * (f64::from(number_of_digits) - 1f64)).ceil() as u32,

     (f64::from(number_of_digits) * log10_b2).ceil() as u32)
}

// Returns number of digits in a number
pub fn number_of_digits(test_number: &Int) -> u32 {
    let mut digit_number = 0;
    let mut d = test_number.clone();
    while d != 0 {
        d = &d / 10;
        digit_number += 1;
    }
    digit_number
}

// LR jacobi algorithm. (d/n)
pub fn jacobi_symbol(d: &Int, n: &Int) -> Result<Int, String> {
    let mut result: Int = Int::one();
    let mut d = d.clone();
    let mut n = n.clone();
    if &n & 1 == 0 {
        return Err("Jacobi symbol are not defined for even n.".to_string());
    }
    if d < 0 {
        d = -d;
        if &n % 4 == 3 {
            result = -result;
        }
    }
    while d != 0 {
        while &d & 1 == 0 {
            d = &d >> 1;
            match i32::from(&(&n % 8)) {
                3 | 5 => result = -result,
                _ => {}
            }
        }
        d = &d ^ &n;
        n = &d ^ &n;
        d = &d ^ &n;
        if &d % 4 == 3 && &n % 4 == 3 {
            result = -result;
        }
        d = &d % &n;
    }
    if n == 1 { Ok(result) } else { Ok(Int::zero()) }
}
