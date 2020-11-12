use std::fs;

fn main() {
    let filepath = "./AND.asm";
    let file = fs::read_to_string(filepath).expect("Failed to read file");
    println!("{}", file);
}
