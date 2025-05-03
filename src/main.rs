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
    append_jumpdest, append_push_jump, get_appended_jumpdest_pos, get_instruction_count,
    get_random_dead_bytecode, modify_push_val,
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

    // 1) Append JUMPDEST at the end

    // Append jumpdest at the end of the bytecode
    append_jumpdest(&mut bytecode);
    // chech if jumpdest has appended
    // println!("jumpdest has appended {}", bytecode);

    // 2) Get all PUSH-JUMP sequence

    let push_jump_seq: Vec<Push_Positions> = find_jump_seq(&bytecode);

    // log all the push-jump sequences
    // println!("push-jump seq: {:?}", push_jump_seq);

    // 3) For each sequence, change the push's param to the newly added JUPDEST's instruction position

    // iterate over all the push-jump seq

    for push_jump in &push_jump_seq {
        // get original jumpdest's position
        let ideal_jumpdest_position: String = push_jump.value_hex.clone();

        // get the byteoffset of appended JUMPDEST
        let appended_jumpdest_pos = get_appended_jumpdest_pos(&bytecode);
        println!("total instructions are: {}", appended_jumpdest_pos);

        // now change the push value to the newly added JUMPDEST's instruction position
        modify_push_val(
            &mut bytecode,
            push_jump.byteoffset_decimal,
            &appended_jumpdest_pos,
        );

        println!("Modified bytecode is {}", bytecode);

        // 3a)

        // 3b)

        // 3c) append push-jump at the end jumping to the original JUMPDEST
        append_push_jump(&mut bytecode, ideal_jumpdest_position);
        println!("Obfuscated Bytecode: {}", bytecode);

        //exit the process
        process::exit(1);
    }
}

/*  OBFUSCATION STEPS
1) append jump dest at the end of the bytecode
2) check for the push-jump seq
3) for each push-jump, change the push's parameter to newly appended jumpdest
    3a) generate dead bytecode and fix the push-jump param according to the total instructions in the bytecode
    3b) append deadbytecode at the end
    3c) append push-jump with correct push value pointing to original jumpdest location.
*/
