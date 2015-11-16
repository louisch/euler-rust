use num::traits::PrimInt;
use std::cmp::max;
use unicode_segmentation::UnicodeSegmentation;

pub fn largest_palindrome(digits: u32) -> u32 {
    let lower = 10.pow(digits - 1);
    let upper = 10.pow(digits);
    let range = lower..upper;

    range.fold(0, |largest, x| {
        match (lower..x + 1).rev().find(|y| is_palindrome(x * y)) {
            Some(y) => max(largest, x * y),
            None => largest
        }
    })
}

fn is_palindrome(n: u32) -> bool {
    n.to_string() ==
        UnicodeSegmentation::graphemes(&n.to_string()[..], true)
        .rev().collect::<String>()
}
