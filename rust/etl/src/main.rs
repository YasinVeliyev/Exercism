use etl::*;
use std::collections::BTreeMap;

fn input_from(v: &[(i32, Vec<char>)]) -> BTreeMap<i32, Vec<char>> {
    v.iter().cloned().collect()
}

fn main() {
    let input = input_from(&[
        (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2, vec!['D', 'G']),
        (3, vec!['B', 'C', 'M', 'P']),
        (4, vec!['F', 'H', 'V', 'W', 'Y']),
        (5, vec!['K']),
        (8, vec!['J', 'X']),
        (10, vec!['Q', 'Z']),
    ]);

    let j = transform(&input);

    println!("{:?}", j)
}
