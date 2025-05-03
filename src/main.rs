// Declare the folder and file as modules
mod constant {
    pub mod opcodes;
}

mod analysis {
    pub mod jump_seq;
}

mod obfuscation {
    pub mod modify_param;
}

// use constant::opcodes::{get_opcode_size, get_opcode_name};

use analysis::jump_seq::{Push_Positions, find_jump_seq};

use obfuscation::modify_param::modify_push_val;

fn main() {
    let mut bytecode: String = String::from("6042600a5652602060005b0033602f01601656003360ff5b00");
    println!("priginal bytecode is {}", bytecode);

    let jump_sequences: Vec<Push_Positions> = find_jump_seq(&bytecode);

    modify_push_val(&mut bytecode, 4);

    println!("mutated bytecode is {}", bytecode);
}
