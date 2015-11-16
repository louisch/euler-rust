use math;

// Return the first factor of n that is not 1 or n itself.
// Return None if no such factor found (i.e. n is prime)
pub fn factor(n: u64) -> Option<(u64, u64)> {
    for x in 2..math::sqrt_u64(n)+1 {
        if n % x == 0 {
            return Some((n / x, x));
        }
    }
    None
}
