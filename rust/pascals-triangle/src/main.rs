use pascals_triangle::*;

fn main() {
    let pt = PascalsTriangle::new(30);
    println!("{:?}", pt.rows());
}
