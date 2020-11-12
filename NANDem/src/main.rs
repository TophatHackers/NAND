use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let file = fs::read(filepath).expect("Failed to read file");
    let mut instructionvec = Vec::<String>::new();

    for rawinstruction in file.iter() {
        //println!("{}", rawinstruction);
        let instruction = format!("{:b}", rawinstruction);
        //println!("{}", instruction);
        instructionvec.push(instruction);
    }

    



}
