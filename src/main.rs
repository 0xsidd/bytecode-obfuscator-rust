use std::process;

// Declare the folder and file as modules
mod constant {
    pub mod opcodes;
}

mod helper {
    pub mod bytecode;
}

mod analysis {
    pub mod jump_seq;
}

use constant::opcodes::{get_opcode_name, get_opcode_size};

use helper::bytecode::{
    self, append_jumpdest, append_push_jump, get_dead_bytecode, get_instruction_count,
    get_last_instruction_position, modify_push_val,
};

use analysis::jump_seq::{Push_Positions, find_jump_seq};

fn main() {
    let mut bytecode: String = String::from("6042600a5652602060005b0033602f01601656003360ff5b00");
    /*
    6042
    600a 2
    56
    52
    6020
    6000
    5b
    00
    33
    602f
    01
    6016 12
    56
    00
    33
    60ff
    5b
    00
    */

    // chech if jumpdest has appended
    // println!("jumpdest has appended {}", bytecode);

    // 1) Get all PUSH-JUMP sequence

    let push_jump_seq: Vec<Push_Positions> = find_jump_seq(&bytecode);

    // log all the push-jump sequences
    // println!("push-jump seq: {:?}", push_jump_seq);

    // 3) For each sequence, change the push's param to the newly added JUPDEST's instruction position
    // iterate over all the push-jump seq

    for push_jump in &push_jump_seq {
        // 2a) Append JUMPDEST at the end

        // Append jumpdest at the end of the bytecode
        append_jumpdest(&mut bytecode);

        // get original jumpdest's position
        let ideal_jumpdest_position: String = push_jump.value_hex.clone();

        // get the byteoffset of appended JUMPDEST
        let appended_jumpdest_pos: i32 = get_last_instruction_position(&bytecode);

        // now change the push value to the newly added JUMPDEST's instruction position
        modify_push_val(
            &mut bytecode,
            push_jump.byteoffset_decimal,
            appended_jumpdest_pos,
            &push_jump.instruction_bits,
        );

        // println!("Modified bytecode is {}", bytecode);

        // 2b) Generate dead bytecode with correct push values

        let last_ins_position: i32 = get_last_instruction_position(&bytecode);
        let dead_bytecode: String = get_dead_bytecode(last_ins_position);
        bytecode = bytecode.clone() + &dead_bytecode;
        // 2c)

        // 2d) append push-jump at the end jumping to the original JUMPDEST
        append_push_jump(&mut bytecode, ideal_jumpdest_position);
        println!("Obfuscated Bytecode: {}", bytecode);

        //exit the process
        process::exit(1);
    }
}

/*  OBFUSCATION STEPS
1) check for the push-jump seq
2) for each push-jump, change the push's parameter to newly appended jumpdest
    2a) append jump dest at the end of the bytecode
    2b) generate dead bytecode and fix the push-jump param according to the total instructions in the bytecode
    2c) append deadbytecode at the end
    2d) append push-jump with correct push value pointing to original jumpdest location.
*/
