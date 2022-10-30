use forth::*;
fn main() {
    let mut f = Forth::new();
    f.eval("1 2 + 4 -");
    println!("{:?}", f.stack());
    println!("{:?}", f.eval("1 2 + 4 -"));
}
