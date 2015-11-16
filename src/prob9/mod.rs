pub fn demo() {
    println!("{}", find_abc());
}

fn find_abc() -> u64 {
    let n = 1000;
    for c in 3..n-2 {
        for b in 2..c {
            if let Some(a) = a_given(b) {
                if is_pythagorean(a, b, c) && a + b + c == n {
                    return a * b * c;
                }
            }
        }
    }
    0
}

// Check whether a, b, and c are a pythagorean triple.
fn is_pythagorean(a: u64, b: u64, c: u64) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

// Given a + b + c = 1000 and a^2 + b^2 = c^2, a can be expressed in terms of b.
fn a_given(b: u64) -> Option<u64> {
    use num::integer::div_rem;
    use num::traits::ToPrimitive;

    let b_i = b.to_i64().unwrap();
    if b_i * 1000 == 500000 || b_i == 1000 {
        return None;
    }
    let nominator = 1000 * b_i - 500000;
    let denominator = b_i - 1000;
    let (div, rem) = div_rem(nominator, denominator);

    if rem == 0 && div > 0 { Some(div.to_u64().unwrap()) } else { None }
}
