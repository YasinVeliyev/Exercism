pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let words = phrase
        .split(|ref ch| [' ', '-', '_'].contains(ch))
        .for_each(|word| {
            if !word.is_empty()
                && (word.to_ascii_uppercase() == word || word.to_ascii_lowercase() == word)
            {
                acronym.push(word.chars().nth(0).unwrap().to_ascii_uppercase());
            } else {
                word.chars().for_each(|char| {
                    if char.is_uppercase() {
                        acronym.push(char);
                    }
                });
            }
        });

    acronym
}

// pub fn abbreviate(phrase: &str) -> String {
//     let mut acronym = String::new();
//     let words = phrase
//         .split(|ch: char| [' ', '-', '_'].contains(&ch))
//         .for_each(|word| {
//             if !word.is_empty()
//                 && (word.to_ascii_uppercase() == word || word.to_ascii_lowercase() == word)
//             {
//                 acronym.push(word.chars().nth(0).unwrap().to_ascii_uppercase());
//             } else {
//                 word.chars().for_each(|char| {
//                     if char.is_uppercase() {
//                         acronym.push(char);
//                     }
//                 });
//             }
//         });
//     acronym
// }

// pub fn abbreviate(phrase: &str) -> String {
//     let mut acronym = String::new();
//     let words = phrase
//         .split(|ch| ch == ' ' || ch == '-' || ch == '_')
//         .for_each(|word| {
//             if !word.is_empty()
//                 && (word.to_ascii_uppercase() == word || word.to_ascii_lowercase() == word)
//             {
//                 acronym.push(word.chars().nth(0).unwrap().to_ascii_uppercase());
//             } else {
//                 word.chars().for_each(|char| {
//                     if char.is_uppercase() {
//                         acronym.push(char);
//                     }
//                 });
//             }
//         });
//     acronym
// }

// pub fn abbreviate(phrase: &str) -> String {
//     let mut acronym = String::new();
//     let ref chars = phrase.chars().collect::<Vec<char>>();
//     let mut delimiter = false;
//     for (index, ch) in chars.iter().enumerate() {
//         if index == 0 {
//             acronym.push(*ch)
//         } else if !ch.is_alphabetic() && *ch != '\'' {
//             delimiter = true
//         } else if chars[index - 1].is_lowercase() && ch.is_uppercase() {
//             acronym.push(*ch)
//         } else if delimiter {
//             delimiter = false;
//             acronym.push(*ch)
//         }
//     }
//     acronym.to_uppercase()
// }

// pub fn abbreviate(phrase: &str) -> String {
//     let mut acronym = String::new();
//     let ref chars = phrase.chars().collect::<Vec<char>>();
//     let delimiters = [' ', '_', '-'];
//     let mut delimiter = false;
//     for (index, ch) in chars.iter().enumerate() {
//         if index == 0 {
//             acronym.push(*ch)
//         } else if delimiters.contains(ch) {
//             delimiter = true
//         } else if chars[index - 1].is_lowercase() && ch.is_uppercase() {
//             acronym.push(*ch)
//         } else if delimiter {
//             delimiter = false;
//             acronym.push(*ch)
//         }
//     }
//     acronym.to_uppercase()
// }
