/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum_numbers = 0;
    let isbn = isbn.replace("-", "").chars().collect::<Vec<char>>();
    if isbn.len() != 10 {
        return false;
    }
    for (index, c) in isbn.iter().enumerate() {
        match c.to_digit(10) {
            Some(a) => {
                sum_numbers += a * (10 - index as u32);
            }
            None => {
                if *c == 'X' && index == 9 {
                    let remainder = sum_numbers % 11;
                    if remainder > 0 {
                        return true;
                    }
                }
                return false;
            }
        };
    }

    sum_numbers % 11 == 0
}
