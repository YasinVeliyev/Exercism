use itertools::Itertools;

use std::collections::HashMap;
use std::iter::Iterator;
pub struct Allergies {
    pub allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergies = HashMap::from([
            (1, Allergen::Eggs),
            (2, Allergen::Peanuts),
            (4, Allergen::Shellfish),
            (8, Allergen::Strawberries),
            (16, Allergen::Tomatoes),
            (32, Allergen::Chocolate),
            (64, Allergen::Pollen),
            (128, Allergen::Cats),
        ]);

        match allergies.get(&score) {
            Some(allergen) => Self {
                allergens: vec![allergen.clone()],
            },
            None => {
                let mut score = score.rem_euclid(256);
                if score == 1 {
                    score = 255;
                }
                let numbers = [1, 2, 4, 8, 16, 32, 64, 128]
                    .into_iter()
                    .filter(|c| c < &score);
                let mut allergens: Vec<Allergen> = Vec::new();
                let len = numbers.clone().count();

                for i in 2..=len {
                    numbers
                        .clone()
                        .permutations(i)
                        .map(|mut s| {
                            s.sort();
                            s
                        })
                        .unique()
                        .for_each(|a| {
                            if a.clone().into_iter().sum::<u32>() == score {
                                for j in &a {
                                    allergens.push(*allergies.get(&j).unwrap())
                                }
                            }
                        });
                }

                Self { allergens }
            }
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}

// use self::Allergen::*;
// #[derive(Debug, PartialEq, Copy, Clone)]
// pub enum Allergen {
//     Eggs = 1 << 0,
//     Peanuts = 1 << 1,
//     Shellfish = 1 << 2,
//     Strawberries = 1 << 3,
//     Tomatoes = 1 << 4,
//     Chocolate = 1 << 5,
//     Pollen = 1 << 6,
//     Cats = 1 << 7,
// }
// const ALLERGENS: [Allergen; 8] = [
//     Eggs,
//     Peanuts,
//     Shellfish,
//     Strawberries,
//     Tomatoes,
//     Chocolate,
//     Pollen,
//     Cats,
// ];
// pub struct Allergies {
//     allergens: u32,
// }
// impl Allergies {
//     pub fn new(n: u32) -> Allergies {
//         Allergies { allergens: n }
//     }
//     pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
//         let allergen = *allergen as u32;
//         println!("{}", self.allergens & allergen == allergen);
//         self.allergens & allergen == allergen
//     }
//     pub fn allergies(&self) -> Vec<Allergen> {
//         ALLERGENS
//             .iter()
//             .filter(|a| self.is_allergic_to(a))
//             .cloned()
//             .collect()
//     }
// }
