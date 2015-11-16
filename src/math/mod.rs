use num::traits::ToPrimitive;

pub mod factors;
pub mod primes;

pub fn sqrt_u64(n: u64) -> u64 {
    n.to_f64().unwrap().sqrt().floor().to_u64().unwrap()
}
