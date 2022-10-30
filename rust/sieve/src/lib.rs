pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = vec![];
    let ref mut v = vec![true; upper_bound as usize - 1];
    for i in 2..=upper_bound / 2 {
        v.into_iter()
            .skip((i as usize - 1) * 2)
            .step_by(i as usize)
            .for_each(|value| {
                *value = false;
            })
    }
    v.into_iter().enumerate().for_each(|(index, value)| {
        if *value {
            primes.push((2..=upper_bound).nth(index).unwrap());
        }
    });
    primes
}
