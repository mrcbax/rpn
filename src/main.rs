extern crate regex;

use std::io;

use regex::Regex;

#[derive(Clone)]
struct Register {
    name: String,
    value: i128,
    empty: bool
}

struct Memory {
    stack: Vec<i128>,
    registers: Vec<Register>,
    stored_regs: Option<Vec<Register>>
}

fn init(memory: Option<Memory>, keep_mem: bool) -> Memory {
    if keep_mem {
        Memory{
            stack: vec!(),
            registers: vec!(
                Register{name: "x".to_string(), value: 0 as i128, empty: true},
                Register{name: "y".to_string(), value: 0 as i128, empty: true},
                Register{name: "z".to_string(), value: 0 as i128, empty: true}),
            stored_regs: memory.unwrap().stored_regs
        }
    } else {
        Memory{
            stack: vec!(),
            registers: vec!(
                Register{name: "x".to_string(), value: 0 as i128, empty: true},
                Register{name: "y".to_string(), value: 0 as i128, empty: true},
                Register{name: "z".to_string(), value: 0 as i128, empty: true}),
            stored_regs: None
        }
    }
}

fn main() {
    let mut memory: Memory = init(None, false);
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = String::from(command.trim());
        let to_store: Option<i128> = match command.parse::<i128>() {
            Ok(o) => Some(o),
            Err(_) => None
        };
        if to_store.is_none() {
            match command.as_str() {
                "clear" => memory = init(Some(memory), true),
                "clmem" => memory = init(None, false),
                "store" => {
                    memory.stored_regs = Some(memory.registers.clone());
                },
                "recal" => {
                    memory.registers = memory.stored_regs.unwrap_or(memory.registers);
                    memory.stored_regs = None;
                },
                "sstk" => {
                    for item in &memory.stack {
                        println!("\t\t\t{}", item);
                    }
                },
                "exit" => break,
                "quit" => break,
                _ => {
                    if Regex::new(r"([0-9]).+").unwrap().is_match(command.as_str()) {
                        match command.split_at(command.len() - 1).0.parse::<i128>() {
                            Ok(o) => {
                                memory = store(memory, o);
                                memory = operate(memory, command.split_at(command.len() - 1).1);
                            },
                            Err(_) => {},
                        }
                    }
                    if Regex::new(r"[-+*/]").unwrap().is_match(command.as_str()) {
                        memory = operate(memory, command.as_str());
                    }
                }
            }
        } else {
            memory = store(memory, to_store.unwrap());
        }
        for i in 0..memory.registers.len() {
            let len = memory.registers.len() - 1;
            println!("{}: {}", memory.registers[len - i].name, memory.registers[len - i].value);
        }
        if memory.stored_regs.is_some() {
            print!("S");
        }
        if !memory.stack.is_empty() {
            print!("H");
        }
        println!("");
    }
}

fn store(mut memory: Memory, to_store: i128) -> Memory {
    let mut is_stored: bool = false;
    for mut register in & mut memory.registers {
        if register.empty {
            register.value = to_store;
            register.empty = false;
            is_stored = true;
            break;
        }
    }
    if !is_stored {
        memory.stack.push(to_store);
    }
    return memory;
}

fn operate(mut memory: Memory, operator: &str) -> Memory{
    let operand_1: i128 = memory.registers[0].value;
    let operand_2: i128 = memory.registers[1].value;
    memory.registers[1].value = memory.registers[2].value;
    if memory.registers[2].empty == true {
        memory.registers[1].empty = true;
    }
    if memory.stack.is_empty() {
        memory.registers[2].value = 0;
        memory.registers[2].empty = true;
    } else {
        memory.registers[2].value = memory.stack.pop().unwrap();
    }
    match operator {
        "+" => memory.registers[0].value = operand_1 + operand_2,
        "-" => memory.registers[0].value = operand_1 - operand_2,
        "*" => memory.registers[0].value = operand_1 * operand_2,
        "/" => memory.registers[0].value = operand_1 / operand_2,
        _ => {}
    }
    return memory;
}
