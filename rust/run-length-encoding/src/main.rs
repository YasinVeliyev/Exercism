use run_length_encoding::*;

fn main() {
    let s = &encode("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB");
    println!("{} :{}", s, decode(&s));
}
