use num;
use num::bigint::BigUint;

pub fn demo(n: BigUint) {
    println!("{}", square_sum_diff(&n));
}

pub fn square_sum_diff(n: &BigUint) -> BigUint {
    use num::bigint::ToBigUint;
    let one = 1.to_biguint().unwrap();
    let n_plus_one = n + &one;
    let square_of_sum: BigUint = num::pow(n * &n_plus_one >> 1, 2);
    let sum_of_squares: BigUint = n * ((n << 1) + &one) * &n_plus_one / 6.to_biguint().unwrap();
    square_of_sum - sum_of_squares
}

pub fn square_sum_diff_iter(n: u64) -> u64 {
    (1..n+1).fold(
        0, |outer_acc, x|
        outer_acc +
            (1..n+1).filter(|y| &x != y).fold(
                0, |inner_acc, y| inner_acc + x * y))
}
