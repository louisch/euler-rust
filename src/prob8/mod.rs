use num::traits::ToPrimitive;

pub fn demo(n: String, num_digits: usize) {
    println!("{}", alg(n, num_digits));
}

fn alg(n: String, num_digits: usize) -> u64 {
    let digits = n.chars().collect::<Vec<char>>();
    let mut largest_product = 0;

    for i in 1..digits.len() - num_digits {
        use std::cmp;
        let current_product = digits[i..i+num_digits].iter()
            .fold(1, |acc, x| acc * x.to_digit(10).unwrap().to_u64().unwrap());
        largest_product = cmp::max(largest_product, current_product);
    }

    largest_product
}
