use std::collections::HashMap;

use parallel_letter_frequency::*;

fn main() {
    let v = vec!["abc"; 1000];
    let mut hm = HashMap::new();
    hm.insert('a', 1000);
    hm.insert('b', 1000);
    hm.insert('c', 1000);
    frequency(&v[..], 4);
}
