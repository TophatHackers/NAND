use std::env;
use std::fs;
use u32;

fn main() {

    let binary = load_binary();
    println!("binary: {:?}", binary);
    emulate_program(binary);
    


}

fn emulate_program(instructions: Vec<String>) {
    
    // -| Init processor

    let mut stack = Vec::<u32>::new();

    let mut registers: [u32; 8] = [0, 0, 2, 3, 0, 0, 0, 0];

    // -|
    
    fn parse_nand(instruction: String, mut registers: [u32;8]) -> [u32;8] {

        let rt = usize::from_str_radix(&instruction[2..5], 2).unwrap();
        let rs = usize::from_str_radix(&instruction[5..8], 2).unwrap();

        registers[1] = !(registers[rt]&registers[rs]);

        registers

    }

    for instruction in instructions {
        println!("Registers: {:?}", registers);
        let op = &instruction[0..2];
        match op {
            "00" => registers = parse_nand(instruction, registers),
            //"01" => parse_sys(instruction, registers),
            //"10" => parse_start(instruction, registers),
            //"11" => parse_end(instruction, registers),
            _ => continue
        };
    }

} 

fn load_binary() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let file = fs::read(filepath).expect("Failed to read file");
    let mut instructionvec = Vec::<String>::new();

    for rawinstruction in file.iter() {
        let instruction = format!("{:0>8b}", rawinstruction);
        instructionvec.push(instruction);
    }

    instructionvec

}