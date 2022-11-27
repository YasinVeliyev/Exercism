pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let s1 = s1.chars();
    let s2: Vec<char> = s2.chars().collect();

    if s1.size_hint().1 == Some(s2.len()) {
        return Some(s1.enumerate().filter(|&(i, v)| s2[i] != v).count());
    }
    None
}
