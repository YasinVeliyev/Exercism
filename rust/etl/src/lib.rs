use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new = BTreeMap::new();
    h.iter().for_each(|(key, value)| {
        value.iter().for_each(|c| {
            new.insert(c.to_ascii_lowercase(), *key);
        })
    });

    new
}
