use isbn_verifier::*;

fn main() {
    is_valid_isbn("3-598-21507-X");

    println!(
        "'{}'",
        "3-598-21508-8"
            .replace("-", "")
            .split("")
            .collect::<String>()
    );
}
