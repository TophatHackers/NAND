use std::{env, fs, collections::HashMap, char};
use u32;
use bit_vec::BitVec;

fn main() {

    let binary = load_binary();
    println!("{:?}", binary);
    //emulate_program(binary)
    


}


fn emulate_program(instructions: Vec<String>) {

    // -| Init processor
    let mut stack = Vec::<u32>::new();

    let mut registers: [u32; 8] = [0, 0, 2, 3, 0, 0, 0, 0,];

    let mut parse_nand = |instruction: String| {
        let rt = u32::from_str_radix(&instruction[2..5], 2).unwrap();
        let rs = u32::from_str_radix(&instruction[5..8], 2).unwrap();
        
        //registers[1] = NAND(rt, rs);
        println!("{}", registers[1]);
        // let and = (registers[rt] & registers[rs]);
        // println!("{} NAND {} == {}", registers[rt], registers[rs], and);
        // registers[1] = !and;
        // println!("NOT {} == {}", and, registers[1]);
        
        
    };

    fn parse_sys(instruction: String) {
        
    }

    fn parse_start(instruction: String) {
        
    }

    fn parse_end(instruction: String) {
        
    }


    println!("{:?}", instructions);

    for instruction in instructions {
        let op = &instruction[0..2];
        match op {
            "00" => parse_nand(instruction),
            "01" => parse_sys(instruction),
            "10" => parse_start(instruction),
            "11" => parse_end(instruction),
            _ => panic!("Invalid instruction format!"),
        };
    }

} 

fn load_binary() -> Vec<BitVec> {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let file = fs::read(filepath).expect("Failed to read file");
    let mut instructionvec = Vec::<BitVec>::new();

    for rawinstruction in file.iter() {
        let bitstring = format!("{:0>8b}", rawinstruction);
        let mut bitvec = BitVec::new();
        for bit in bitstring.chars() {
            if bit == '1'{
                bitvec.push(true);
            }
            else {
                bitvec.push(false)
            }
        }
        instructionvec.push(bitvec);
    }

    instructionvec

}