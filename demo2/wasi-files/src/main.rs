use std::fs;

fn main() {
    let content = fs::read_to_string("text.txt").expect("Could not read file");

    println!("Content:\n{content}");
}
