use num::bigint::BigUint;
use num::traits::ToPrimitive;
use num::one;
use num::zero;

pub struct Fib {
    prev2: [BigUint; 2],
}

impl Fib {
    pub fn new() -> Fib {
        Fib { prev2: [zero(), zero()] }
    }
}

impl Iterator for Fib {
    type Item = BigUint;

    fn next(&mut self) -> Option<BigUint> {
        if self.prev2[1] == zero() {
            self.prev2[1] = one();
        } else {
            let new_fib = self.prev2[0].clone() + self.prev2[1].clone();
            self.prev2[1] = self.prev2[0].clone();
            self.prev2[0] = new_fib;
        }
        Some(self.prev2[0].clone())
    }
}

pub fn fib(n: u64) -> Vec<BigUint> {
    Fib::new().take(n.to_usize().unwrap()).collect::<Vec<BigUint>>()
}
