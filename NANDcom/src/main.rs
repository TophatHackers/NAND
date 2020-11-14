use std::fs;
extern crate bit_vec;
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::process;

struct Paths{
    filepath: String,
    define_filepath: String,
    output_filepath: String
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let paths= Paths::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let file = fs::read_to_string(paths.filepath).expect("Failed to read input file");
    let file = file
        .split("\n")
        .filter(|l| !l.is_empty()).map(|l|l.to_string())
        .collect::<Vec<String>>();

    let define_file = fs::read_to_string(paths.define_filepath).expect("Failed to read define file");
    
    let definitions = load_definition(&define_file);

    let file = replace_macro(&file, &definitions);

    for line in &file{
        println!("{}",line);
    }

    let bit_vector = compile(file);

    let mut file = fs::File::create(paths.output_filepath).unwrap();
    file.write_all(&bit_vector);
}

fn set_bits(bitvec: &mut Vec<u8>, index: usize, bits: Vec<u8>) {
    for (i, b) in bits.into_iter().enumerate() {
        bitvec[7 - index + i] = b;
    }
}

fn compile(file: Vec<String>) -> Vec<u8> {
    let mut bvVector: Vec<u8> = Vec::<u8>::new();
    for line in file {
        let mut bv: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        match split_line[0] {
            "sys" => {
                set_bits(&mut bv, 7, vec![0, 1]);
                match split_line[1] {
                    "STACK" => {
                        set_bits(&mut bv, 5, vec![0, 0]);
                        match split_line[2] {
                            "POP" => set_bits(&mut bv, 3, vec![1]),
                            "PUSH" => set_bits(&mut bv, 3, vec![0]),
                            _ => {
                                println!("STACK can only PUSH or POP");
                                process::exit(1);
                            }
                        }
                        let registry_number = get_registry_number(split_line[3]);
                        match registry_number {
                            Ok(v) => set_bits(&mut bv, 2, v),
                            Err(s) => {
                                println!("{}", s);
                                break;
                            }
                        }
                    }
                    "READ" => {
                        set_bits(&mut bv, 5, vec![0, 1]);
                        let registry_number = get_registry_number(split_line[2]);
                        match registry_number {
                            Ok(v) => set_bits(&mut bv, 3, v),
                            Err(s) => {
                                println!("{}", s);
                                break;
                            }
                        }
                    }
                    "WRITE" => {
                        set_bits(&mut bv, 5, vec![1, 0]);
                        let registry_number = get_registry_number(split_line[2]);
                        match registry_number {
                            Ok(v) => set_bits(&mut bv, 3, v),
                            Err(s) => {
                                println!("{}", s);
                                break;
                            }
                        }
                    }
                    _ => {
                        println!("no such syscall");
                        break;
                    }
                }
            }
            "START" => {
                set_bits(&mut bv, 7, vec![1, 0]);
                let registry1_number = get_registry_number(split_line[1]);
                match registry1_number {
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s) => {
                        println!("{}", s);
                        break;
                    }
                }

                let registry2_number = get_registry_number(split_line[2]);
                match registry2_number {
                    Ok(v) => set_bits(&mut bv, 2, v),
                    Err(s) => {
                        println!("{}", s);
                        break;
                    }
                }
            }
            "END" => {
                set_bits(&mut bv, 7, vec![1, 0]);
                let registry1_number = get_registry_number(split_line[1]);
                match registry1_number {
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s) => {
                        println!("{}", s);
                        process::exit(1);
                    }
                }
            }
            "NAND" => {
                set_bits(&mut bv, 7, vec![0, 0]);
                let registry1_number = get_registry_number(split_line[1]);
                match registry1_number {
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s) => {
                        println!("{}", s);
                        process::exit(1);
                    }
                };
                let registry2_number = get_registry_number(split_line[2]);
                match registry2_number {
                    Ok(v) => set_bits(&mut bv, 2, v),
                    Err(s) => {
                        println!("{}", s);
                        process::exit(1);
                    }
                }
            }
            "BIT" =>{
                set_bits(&mut bv, 7, vec![1,1]);
                match split_line[1]{
                    "READ"=>{
                        set_bits(&mut bv, 5, vec![0])
                    },
                    "WRITE" =>{
                        set_bits(&mut bv, 5, vec![1])
                    },
                    _ => {
                        panic!("BIT has no such command {}",split_line[1]);
                    }
                }

                let imm=format!("{:0>5b}",split_line[2].parse::<u8>().unwrap()).chars().map(|c| c.to_digit(2).unwrap() as u8).collect::<Vec<u8>>();
                set_bits(&mut bv, 4, imm);

            }
            _ => println!("unknown command {}", split_line[0]),
        }
        let mut number: u8 = 0;
        let base: u8 = 2;
        for (i, b) in bv.iter().enumerate() {
            number += base.pow((7 - i) as u32) * b;
         
        }
        
        bvVector.push(number);
    }

    return bvVector;
}

fn get_registry_number(registry: &str) -> Result<Vec<u8>, String> {
    let mut index_registry;
    match registry {
        "pc" => index_registry = 0,
        "rn" => index_registry = 1,
        "r0" => index_registry = 2,
        "r1" => index_registry = 3,
        "r2" => index_registry = 4,
        "r3" => index_registry = 5,
        "r4" => index_registry = 6,
        "r5" => index_registry = 7,
        _ => {
            println!("no such registry {}", registry);
            return Err(String::from("no such registry"));
        }
    }
    let mut s = format!("{:0>3b}", index_registry);

    return Ok(s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>());
}

fn replace_macro<'a>(
    nand_file: &'a Vec<String>,
    definitions: &'a HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut replaced_file = Vec::<String>::new();

    for line in nand_file {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        if definitions.contains_key(split_line[0]) {
            let  mut definition = definitions.get(split_line[0]).unwrap().clone();
            let number_of_args = definition[0].parse::<usize>().unwrap();
            definition.remove(0);
           

            if split_line.len() - 1 == number_of_args {
                let mut replaced_lines = definition.clone();
                let args_str=definition[0].clone();
                let args: Vec<&str> = args_str.split_whitespace().collect();
                definition.remove(0);
                replaced_lines.remove(0);
                
                let mut counter=0;
                loop{
                    //println!("{}",replaced_lines[counter]);
                    if definitions.contains_key(replaced_lines[counter].split_whitespace().next().unwrap()){
                        let macro_definition=replace_macro(&replaced_lines, definitions);
                        
                        replaced_lines.remove(counter);
                        replaced_lines.splice(counter..counter, macro_definition);
                    }

                    for i in 1..split_line.len() {
                        let reg_to_replace = args[i-1];
                        replaced_lines[counter] = replaced_lines[counter].replace(reg_to_replace, split_line[i]);
                        
                    }

                    if counter==replaced_lines.len()-1{
                        break;
                    }
                    counter+=1;
                }
                
                for i in 0..replaced_lines.len() {
                    replaced_file.push(replaced_lines[i].clone());
                }
            }
        } else {
            replaced_file.push(line.clone());
        }
    }
    //println!("{:?}", replaced_file);
    return replaced_file;
}

fn load_definition(define_file: &String) -> HashMap<String, Vec<String>> {

    let define_file = define_file
    .split("\n")
    .filter(|l| !l.is_empty())
    .collect::<Vec<&str>>();


    let mut in_definition = false;
    let mut definition: Vec<String>=vec![];
    let mut definition_name: String = String::new();
    let mut definitions: HashMap<String, Vec<String>> = HashMap::new();

    for line in define_file {
        let mut split_lines: Vec<&str> = line.split_whitespace().collect();
        if split_lines[0] == ".end_define" {
            in_definition = false;
            definitions.insert(definition_name.clone(), definition.clone());

        } else if in_definition == true {
            //definition = format!("{}\n{}", definition, line)
            definition.push(line.to_string())
        } else if split_lines[0] == ".define" {

            definition = Vec::new();
            definition_name = String::new();

            in_definition = true;
            definition_name = split_lines[1].to_string();
            let number_args = split_lines.len() - 2;

            definition.push(number_args.to_string());
            split_lines.remove(0);
            split_lines.remove(0);

            
            definition.push(split_lines.join(" ").to_string());
            

        }
    }

    
    // let mut found_macro=true;
    // while found_macro {
    //     found_macro=false;
    //     let temp_def= definitions.clone();
    //     for def in &mut definitions{
    //         let new_def=replace_macro(&def.1, &temp_def);
    //         if &new_def!=def.1 {
    //             *def.1=new_def;
    //             found_macro=true;
    //         }
    //     }

    // }
    
        println!("{:?}",definitions);
    return definitions;
}

impl Paths{
    fn new(args: &[String])-> Result<Paths,String>{
        let mut filepath: String = String::from("AND.asm");
        let mut output_filepath: String = String::from("a.nand");
        let mut define_filepath: String = String::from( "define.asm");
    
        if args.len() > 1 {
            for (i, arg) in args.iter().enumerate() {
                if arg.chars().next().unwrap() == '-' {
                    match arg.chars().nth(1).unwrap() {
                        'o' => {
                            if i >= args.len() - 1 {
                                return Err(String::from("Please specify output name"));
                            }
                            output_filepath = args[i + 1].clone();
                        }
                        _ => return Err(String::from("No such flag")),
                    }
                } else {
                    if arg==&output_filepath{
                        continue;
                    }
                    filepath = arg.clone();
                }
            }
        }
    
        return Ok(Paths{filepath, define_filepath,output_filepath});
    }
}
