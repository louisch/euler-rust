use math::primes;

pub fn demo(n: u64) {
    println!("{}", primes::primes(n).last().unwrap());
}
