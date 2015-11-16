use math::primes;
use std::cmp;
use std::collections::HashMap;

pub fn demo(n: u64) {
    println!("{:?}", lcm(n));
}

fn lcm(n: u64) -> u64 {
    let mut common: HashMap<u64, u64> = HashMap::new();

    for i in 2..n {
        let mut i_primes: HashMap<u64, u64> = HashMap::new();
        for prime in primes::prime_factors(i).into_iter() {
            let count = i_primes.entry(prime).or_insert(0);
            *count += 1;
        }

        for (prime, count) in i_primes.into_iter() {
            let common_count = common.entry(prime).or_insert(count);
            *common_count = cmp::max(common_count.clone(), count);
        }
    }

    common.iter().fold(1, |acc, (prime, count)| {
        use num::traits::ToPrimitive;
        acc * prime.pow(count.clone().to_u32().unwrap())
    })
}
