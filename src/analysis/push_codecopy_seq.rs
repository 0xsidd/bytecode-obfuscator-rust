use crate::constant::opcodes;
use crate::helper::bytecode::modify_push_val;
use crate::helper::bytecode::{get_instruction_at_index, get_last_instruction_position};

/*
Find PUSHx -> x1 -> x2 -> x3 -> CODECOPY sequence and return index of push to modify
*/

pub fn update_runtime_offset(creation_code: &mut String, runtime_bytecode: &String) {
    let (push_index, push_instruction) = get_push_codecopy_index(creation_code).unwrap();
    let updated_push_val = calculate_runtime_offset(runtime_bytecode) + 1;

    modify_push_val(
        creation_code,
        push_index,
        updated_push_val,
        &push_instruction,
    );
}

fn get_push_codecopy_index(creation_code: &mut String) -> Option<(i32, String)> {
    // Checks for PUSHx -> x1 -> x2 -> x3 -> CODECOPY sequence in the bytecode and retuens its
    let mut skip_to_index: i32 = 0;
    let mut instruction_position: i32 = 0;

    for (index, _) in creation_code.chars().enumerate() {
        if index as i32 == skip_to_index && index % 2 == 0 {
            let current_instruction: String = creation_code[index..index + 2].to_string();

            let current_instruction_size: i32 =
                opcodes::get_opcode_size(&current_instruction).unwrap() as i32;

            skip_to_index = index as i32 + current_instruction_size;

            instruction_position += 1;
            // println!("Ins: {}, pos: {}",current_instruction,instruction_position);

            let current_plus_four_ins =
                get_instruction_at_index(&creation_code, instruction_position + 4).unwrap();

            // println!("Current: {} current plus 4: {}",current_instruction,current_plus_four_ins);
            if (current_instruction == String::from("5f") || current_instruction.starts_with("6"))
                && current_plus_four_ins == String::from("39")
            {
                return Some((index as i32, current_instruction));
            }
        }
    }
    return None;
}

// bytecode: &mut creation_code
// push_byte_offset: byteoffset_decimal is indec
// replacement_value: to be calculated
// instruction: current_instruction

pub fn calculate_runtime_offset(runtime_bytecode: &String) -> i32 {
    let runtime_byte_length = get_last_instruction_position(runtime_bytecode);
    return runtime_byte_length;
}
