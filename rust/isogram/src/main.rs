use isogram::*;

fn main() {
    check("Emily Jung Schwartzkopf");
    println!(
        "{}",
        "Emily Jung Schwartzkopf"
            .to_ascii_lowercase()
            .trim_matches(|c: char| !c.is_ascii_alphabetic())
    );

    println!("{}", ' '.is_ascii_alphabetic())
}
