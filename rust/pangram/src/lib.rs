/// Determine whether a sentence is a pangram.
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let mut set = HashSet::new();
    sentence.to_ascii_lowercase().chars().for_each(|c| {
        if c.is_alphabetic() {
            set.insert(c);
        }
    });
    set.len() >= 26
}
