pub fn build_proverb(list: &[&str]) -> String {
    let mut string = String::new();
    for i in 1..list.len() {
        string += &format!("For want of a {} the {} was lost.\n", list[i - 1], list[i]);
    }
    if list.len() != 0 {
        string += &format!("And all for the want of a {}.", list[0]);
    }

    string
}
