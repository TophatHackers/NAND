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

    static mut stack:Vec<u32> = Vec::<u32>::new();

    static mut registers: [u32; 8] = [0, 0, 2, 3, 0, 0, 0, 0];

    // -|
    
    fn parse_nand(instruction: String) {

        let rt = usize::from_str_radix(&instruction[2..5], 2).unwrap();
        let rs = usize::from_str_radix(&instruction[5..8], 2).unwrap();

        unsafe { registers[1] = !(registers[rt]&registers[rs] )};

    }

    fn parse_sys(instruction: String) {

        let id = &instruction[2..4];
        match id {
            "00" => {
                let stacktype = &instruction[4..5];
                let rs = usize::from_str_radix(&instruction[5..8], 2).unwrap();
                if stacktype == "0" {
                    unsafe { stack.push(registers[rs]) }
                }
                else {
                    unsafe { registers[rs] = stack.pop().expect("Tried popping from an empty stack!") }
                }
            },
            "01" => {

            }

        }
    }



    for instruction in instructions {
        unsafe {
            println!("Registers: {:?}", registers);
            println!("Stack: {:?}", stack);
        }

        let op = &instruction[0..2];
        match op {
            "00" => parse_nand(instruction),
            "01" => parse_sys(instruction),
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