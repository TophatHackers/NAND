use std::{env, fs, io, convert::TryInto};
use u32;

fn main() {

    let binary = load_binary();
    println!("binary: {:?}", binary);
    emulate_program(binary);
    


}

fn emulate_program(binary: Vec<String>) {
    
    // -| Init processor

    static mut STACK:Vec<u32> = Vec::<u32>::new();

    static mut REGISTERS: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    // -|

    fn run_process(binary: Vec<String>) {

        fn parse_instruction(instruction: String) {
            unsafe {
                println!("REGISTERS: {:?}", REGISTERS);
                println!("STACK: {:?}", STACK);
                println!("PROCESS: {:?}", INSTRUCTIONS);
            }
    
            let op = &instruction[0..2];
            match op {
                "00" => parse_nand(instruction),
                "01" => parse_sys(instruction),
                "10" => unsafe { REGISTERS[0] += 1; },
                "11" => unsafe { REGISTERS[0] += 1; },
                //"10" => parse_start(instruction),
                //"11" => parse_end(instruction, REGISTERS),
                _ => return
            };
        }

        static mut INSTRUCTIONS:Vec<String> = Vec::<String>::new();
        unsafe { INSTRUCTIONS = binary.clone(); }

        unsafe {

            while REGISTERS[0] < INSTRUCTIONS.len().try_into().expect("File size too large!") {
                parse_instruction(INSTRUCTIONS[REGISTERS[0] as usize].clone());
            }
            println!("Reached end of process for {:?}", INSTRUCTIONS);
        }
        
    }
    
    fn parse_nand(instruction: String) {

        let rt = usize::from_str_radix(&instruction[2..5], 2).unwrap();
        let rs = usize::from_str_radix(&instruction[5..8], 2).unwrap();

        unsafe { 
            REGISTERS[1] = !(REGISTERS[rt]&REGISTERS[rs]);
            REGISTERS[0] += 1;
        }
        

    }

    fn parse_sys(instruction: String) {

        let id = &instruction[2..4];
        match id {
            "00" => {
                let STACKtype = &instruction[4..5];
                let rs = usize::from_str_radix(&instruction[5..8], 2).unwrap();
                if STACKtype == "0" {
                    unsafe { STACK.push(REGISTERS[rs]) }
                }
                else {
                    unsafe { REGISTERS[rs] = STACK.pop().expect("Tried popping from an empty STACK!") }
                }
            },
            "01" => {
                let rs = usize::from_str_radix(&instruction[4..7], 2).unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input!");
                let input = input.trim().parse::<u32>().expect("Expected u32 integer!");
                unsafe { REGISTERS[rs] = input }
            }
            "10" => {
                let rs = usize::from_str_radix(&instruction[4..7], 2).unwrap();
                unsafe { println!("{}", REGISTERS[rs] ) }
            }
            _ => panic!("Invalid instruction at {}!", instruction),
        };

        unsafe { REGISTERS[0] += 1; }
    }

    //fn parse_start()



    run_process(binary);

    println!("Reached EOF!");

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