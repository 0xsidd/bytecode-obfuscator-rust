use crate::analysis::jump_seq::{PushPositions, find_jump_seq};

use crate::helper::bytecode::{
    append_jumpdest, append_push_jump, get_dead_bytecode, get_last_instruction_position,
    modify_push_val,rm_zero_x
};

pub fn obfuscate(bytecode: &mut String) {

    // Remove 0x from the start

    rm_zero_x(bytecode);
    println!("0x removed: {}",bytecode);
    // 1) Get all PUSH-JUMP sequence
    let push_jump_seq: Vec<PushPositions> = find_jump_seq(&bytecode.clone());

    // 3) For each sequence, change the push's param to the newly added JUPDEST's instruction position
    // iterate over all the push-jump seq

    for push_jump in &push_jump_seq {
        // 2a) Append JUMPDEST at the end

        // Append jumpdest at the end of the bytecode
        append_jumpdest(bytecode);

        // get original jumpdest's position
        let ideal_jumpdest_position: String = push_jump.value_hex.clone();

        // get the byteoffset of appended JUMPDEST
        let appended_jumpdest_pos: i32 = get_last_instruction_position(&bytecode);

        // now change the push value to the newly added JUMPDEST's instruction position
        modify_push_val(
            bytecode,
            push_jump.byteoffset_decimal,
            appended_jumpdest_pos,
            &push_jump.instruction_bits,
        );

        // 2b) Generate dead bytecode with correct push values

        let last_ins_position: i32 = get_last_instruction_position(&bytecode);
        let dead_bytecode: String = get_dead_bytecode(last_ins_position);
        bytecode.push_str(&dead_bytecode);
        // 2c)

        // 2d) append push-jump at the end jumping to the original JUMPDEST
        append_push_jump(bytecode, ideal_jumpdest_position);
    }
}
