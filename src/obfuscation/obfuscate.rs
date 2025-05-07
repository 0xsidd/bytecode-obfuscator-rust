use crate::analysis::code_type::seperate_bytecode;
use crate::analysis::jump_seq::{PushPositions, find_jump_seq};
use crate::analysis::push_codecopy_seq::update_init_push;

use crate::helper::bytecode::{
    append_jumpdest, append_push_jump, get_dead_bytecode, get_last_instruction_position,
    modify_push_val, rm_zero_x,
};

/*  OBFUSCATION STEPS
1) seperate out initcode from the runtime bytecode
2) in the runtime bytecode check for the push-jump seq
3) for each push-jump, change the push's parameter to newly appended jumpdest
    3a) append jump dest at the end of the bytecode
    3b) generate dead bytecode and fix the push-jump param according to the total instructions in the bytecode
    3c) append deadbytecode at the end
    3d) append push-jump with correct push value pointing to original jumpdest location.
4) update runtime length in the initcode
*/

pub fn obfuscate(bytecode: &mut String, max_iterations: usize) {
    // Remove 0x from the start
    rm_zero_x(bytecode);

    // 1) seperate init code and runtime code
    let (init_code, mut runtime_bytecode) =
        seperate_bytecode(&bytecode).unwrap_or_else(|| (String::new(), bytecode.clone()));

    // 2) Get all PUSH-JUMP sequence
    let push_jump_seq: Vec<PushPositions> = find_jump_seq(&runtime_bytecode.clone());

    // 3) For each sequence, change the push's param to the newly added JUPDEST's instruction position
    // iterate over all the push-jump seq

    for push_jump in push_jump_seq.iter().take(max_iterations) {
        // 3a) Append JUMPDEST at the end

        // Append jumpdest at the end of the bytecode
        append_jumpdest(&mut runtime_bytecode);

        // get original jumpdest's position
        let ideal_jumpdest_position: String = push_jump.value_hex.clone();

        // get the byteoffset of appended JUMPDEST
        let appended_jumpdest_pos: i32 = get_last_instruction_position(&runtime_bytecode);

        // now change the push value to the newly added JUMPDEST's instruction position
        modify_push_val(
            &mut runtime_bytecode,
            push_jump.byteoffset_decimal,
            appended_jumpdest_pos,
            &push_jump.instruction_bits,
        );

        // 3b) Generate dead bytecode with correct push values

        let last_ins_position: i32 = get_last_instruction_position(&runtime_bytecode);
        let dead_bytecode: String = get_dead_bytecode(last_ins_position);

        // 3c) append dead bytecode at the end
        runtime_bytecode.push_str(&dead_bytecode);

        // 3d) append push-jump at the end jumping to the original JUMPDEST
        append_push_jump(&mut runtime_bytecode, ideal_jumpdest_position);
    }

    // concatenate init code and runtime code
    bytecode.clear();
    bytecode.push_str(&init_code);
    bytecode.push_str(&runtime_bytecode);

    // 4) update runtime length in the initcode
    update_init_push(bytecode, &runtime_bytecode);
    // println!("{}",bytecode);
}
