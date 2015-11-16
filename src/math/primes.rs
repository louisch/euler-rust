use math;
use math::factors;

// Return whether n is prime or not.
// This algorithm is based on the one given by Project Euler's solution to q7.
pub fn is_prime(n: u64) -> bool {
    n != 1 && n == 2 ||
        n % 2 != 0 && n < 9 ||
        n % 3 != 0 &&
        (5..math::sqrt_u64(n) + 1).all(|x| n % x != 0 && n % (x + 2) != 0)

}

// Find all the prime factors of n
pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut others = vec![n];

    loop {
        match others.pop() {
            Some(x) => match factors::factor(x) {
                Some((factor1, factor2)) => {
                    others.push(factor1);
                    others.push(factor2);
                }
                None => primes.push(x),
            },
            None => break,
        }
    }

    primes
}

// Find the first n primes.
pub fn primes(n: u64) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }

    let mut primes = vec![2];
    let mut current = 3;
    for _ in 0..n {
        while !is_prime(current) {
            current += 2;
        }
        primes.push(current);
        current += 2;
    }
    primes
}

// Find all primes below n.
pub fn primes_below(n: u64) -> Vec<u64> {
    use std::iter::repeat;

    let n_sqrt = math::sqrt_u64(n);
    // Fill up first segment
    let (mut prime, is_primes) = eratosthenes_sieve(n_sqrt);



    primes
}

pub fn eratosthenes_sieve(n: u64) -> (Vec<u64>, Vec<bool>) {
    let mut primes = Vec::new();
    let mut is_prime = repeat(true).take(n as usize).collect::<Vec<bool>>();
    for x in (2..n).filter(|x| x % 2 != 2) {
        if is_prime[x] {
            for multiple in (x .. n/x - 1).map(|y| x * y) {
                is_prime[multiple] = false;
            }
        }
    }
    (prime, is_primes)
}
