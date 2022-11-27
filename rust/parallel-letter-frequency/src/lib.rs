use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let map = Arc::new(Mutex::new(HashMap::<char, usize>::new()));

    let chunks = input.chunks(input.len() / worker_count + 1);

    let mut handlers = vec![];
    for chunk in chunks {
        let map = Arc::clone(&map);
        let input: String = chunk
            .into_iter()
            .map(|s| {
                s.chars()
                    .filter(|ch| ch.is_alphabetic())
                    .collect::<String>()
            })
            .fold(String::new(), |f, s| f + &s.to_lowercase());

        handlers.push(thread::spawn(move || {
            let mut map = map.lock().unwrap();
            input.chars().for_each(|ch| {
                *map.entry(ch).or_default() += 1;
            })
        }));
    }
    handlers
        .into_iter()
        .for_each(|handler| handler.join().unwrap());

    let map = match map.lock() {
        Ok(value) => HashMap::from_iter(value.clone().into_iter()),
        Err(_) => panic!("Error"),
    };
    map
}
