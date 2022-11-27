use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    let binding = candidate.to_ascii_lowercase();
    binding
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| set.insert(c))
}
