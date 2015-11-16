pub mod fib;

use prob2::fib::Fib;
use num::bigint::BigUint;
use num::bigint::ToBigUint;
use num::zero;

fn solve(upper_limit: u64) -> BigUint {
    let mut sum = zero::<BigUint>();
    let mut counter = 0;
    for x in Fib::new().take_while(|x| x < &upper_limit.to_biguint().unwrap()) {
        if counter == 3 {
            sum = &sum + &x;
            counter = 0;
        }
        counter += 1;
    }
    sum
}

pub fn demo(n: u64) {
    println!("{:?}", solve(n));
}
