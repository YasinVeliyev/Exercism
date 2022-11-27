use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    for a in 1..sum / 3 {
        let b = (sum.pow(2) - 2 * sum * a) / (2 * (sum - a));
        let c = sum - a - b;
        if a.pow(2) + b.pow(2) == c.pow(2) {
            let mut l = [a, b, c];
            l.sort();
            set.insert(l);
        }
    }

    set
}
