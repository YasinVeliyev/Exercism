/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let str_value = value.to_string();
        let rev_str_value = str_value.chars().rev().collect::<String>();
        if str_value == rev_str_value {
            return Some(Self(value));
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min >= max {
        return None;
    }
    let max_sqrt = max.pow(2);
    let min_sqrt = min.pow(2);

    let mut max_vec = vec![];
    let mut min_vec = vec![];
    while max_sqrt % 10 != 0 {
        max_vec.push(max_sqrt % 10);
    }
    max_vec.reverse();
    while min_sqrt % 10 != 0 {
        min_vec.push(min_sqrt % 10);
    }
    min_vec.reverse();

    let index = max_vec.len();
    if index % 2 == 0 {
        let m_vec = max_vec.clone();
        
    }

    let mut t = (1, 1);

    println!("min:{} max:{}", min, max);

    println!("Min: {:?} Max: {:?}", t.0, t.1);

    None
}
