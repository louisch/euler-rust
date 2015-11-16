mod palindromes;

pub fn demo(digits: u32) {
    println!("{:?}", palindromes::largest_palindrome(digits));
}
