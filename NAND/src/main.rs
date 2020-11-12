use std::fs;
extern crate bit_vec;
use std::io::prelude::*;

use bit_vec::BitVec;
fn main() {

    let filepath = "./AND.asm";
    let file = fs::read_to_string(filepath).expect("Failed to read file");

    let cleanedFile = file
    .split("\n")
    .filter(|l| !l.is_empty())
    .collect::<Vec<&str>>();


    let define_filepath= "./define.asm";

    let defineFile=fs::read_to_string(filepath).expect("Failed to read file");



 
    let bvVector = compile(cleanedFile);
   
    let mut file = fs::File::create("./a.nand").unwrap();
    file.write_all(&bvVector);
    
}

fn set_bits( bitvec:&mut Vec<u8>, index: usize, bits: Vec<u8>) {
    for (i, b) in bits.into_iter().enumerate() {
        
        bitvec[7-index+i]=b;
    }
}

fn compile(file:Vec<&str>)-> Vec<u8>{
    let mut bvVector:Vec<u8>= Vec::<u8>::new();
    for line in file {
        let mut bv:Vec<u8>= vec![0,0,0,0,0,0,0,0];
        
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        match split_line[0] {
            "sys" => {
                set_bits(&mut bv, 7,vec![0,1]);
                match split_line[1] {
                    "STACK" => {
                        set_bits(&mut bv, 5, vec![0,0]);
                        match split_line[2]{
                            "POP" => set_bits(&mut bv, 3, vec![1]),
                            "PUSH" => set_bits(&mut bv, 3, vec![0]),
                            _ => {
                                println!("STACK can only PUSH or POP");
                                break;
                            }
                        }
                        let registry_number=get_registry_number(split_line[3]);
                        match registry_number {
                            Ok(v) => set_bits(&mut bv, 2, v),
                            Err(s)=> {
                                println!("{}",s);
                                break;
                            }
                        }
                        
                    },
                    "READ" => {
                        set_bits(&mut bv, 5, vec![0,1]);
                        let registry_number= get_registry_number(split_line[2]);
                        match registry_number{
                            Ok(v) => set_bits(&mut bv, 3, v),
                            Err(s)=> {
                                println!("{}",s);
                                break;
                            },
                        }
                    }
                    "WRITE" => {
                        set_bits(&mut bv, 5, vec![1,0]);
                        let registry_number= get_registry_number(split_line[2]);
                        match registry_number{
                            Ok(v) => set_bits(&mut bv, 3, v),
                            Err(s)=> {
                                println!("{}",s);
                                break;
                            },
                        }
                    }
                    _ => {
                        println!("no such syscall");
                        break;
                    }
                }
            }
            "START" => {
                set_bits(&mut bv, 7,vec![1,0]); 
                let registry1_number = get_registry_number(split_line[1]);
                match registry1_number{
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s)=> {
                        println!("{}",s);
                        break;
                    },
                }

                let registry2_number = get_registry_number(split_line[2]);
                match registry2_number{
                    Ok(v) => set_bits(&mut bv, 2, v),
                    Err(s)=> {
                        println!("{}",s);
                        break;
                    },
                }
            }
            "END" => {
                set_bits(&mut bv, 7,vec![1,1]); 
                let registry1_number = get_registry_number(split_line[1]);
                match registry1_number{
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s)=> {
                        println!("{}",s);
                        break;
                    },
                }
            }
            "NAND" =>{
                set_bits(&mut bv, 7,vec![0,0]);
                let registry1_number=get_registry_number(split_line[1]);
                match registry1_number{
                    Ok(v) => set_bits(&mut bv, 5, v),
                    Err(s)=> {
                        println!("{}",s);
                        break;
                    },
                };
                let registry2_number=get_registry_number(split_line[2]);
                match registry2_number{
                    Ok(v) => set_bits(&mut bv, 2, v),
                    Err(s)=> {
                        println!("{}",s);
                        break;
                    },
                }
            }
            _ => println!("unknown command"),
        }
        let mut number:u8=0;
        let base: u8 = 2;
        for (i,b) in bv.iter().enumerate(){
            number += base.pow((7-i) as u32)*b
        }
        bvVector.push(number);
    }

    return bvVector

}

fn get_registry_number(registry:&str) -> Result<Vec<u8>,String>{
    let mut index_registry;
    match registry {
        "pc" => index_registry=0,
        "rn" =>index_registry=1,
        "r0" =>index_registry=2,
        "r1" =>index_registry=3,
        "r2" =>index_registry=4,
        "r3" =>index_registry=5,
        "r4" =>index_registry=6,
        "r5" =>index_registry=7,
        _ => {
            println!("no such registry");
            return Err(String::from("no such registry"));
        },
    }
    
    let mut s = format!("{:0>3b}", index_registry);
    
    
    return Ok(s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
}

fn replace_macro(nand_file: Vec<&str>, define_file:Vec<&str>){

}