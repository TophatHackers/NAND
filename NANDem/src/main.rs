use std::env;
use std::fs;

fn main() {

    let instructions = load_binary();

    


}

fn emulate_program(instructions: Vec<String>) {
    
    

    for instruction in instructions {
        let op = &instruction[0..2];
        match op {
            "00" => parse_nand(instruction),
            "01" => parse_sys(instruction),
            "10" => parse_start(instruction),
            "11" => parse_end(instruction),
        };
    }

} 

fn load_binary() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let file = fs::read(filepath).expect("Failed to read file");
    let mut instructionvec = Vec::<String>::new();

    for rawinstruction in file.iter() {
        println!("{}", rawinstruction);
        let instruction = format!("{:0>8b}", rawinstruction);
        println!("{}", instruction);
        instructionvec.push(instruction);
    }

    instructionvec

}