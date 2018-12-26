use std::io;

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
                _ => println!("Command: {} not understood.", command)
            }
        } else {
            let mut is_stored: bool = false;
            for mut register in & mut memory.registers {
                if register.empty {
                    register.value = to_store.unwrap();
                    register.empty = false;
                    is_stored = true;
                    break;
                }
            }
            if !is_stored {
                memory.stack.push(to_store.unwrap());
            }
        }
        for register in &memory.registers {
            println!("{}: {}", register.name, register.value);
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
