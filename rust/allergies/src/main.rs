use allergies::Allergies;
fn main() {
    let allergies = Allergies::new(509).allergies();
    println!("{:?}", allergies);
}
