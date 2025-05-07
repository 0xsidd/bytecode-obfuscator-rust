use crate::constant::opcodes;

pub fn seperate_bytecode(bytecode: &String) -> Option<(String, String)> {
    let mut skip_to_index: i32 = 0;
    let mut init_code_seperation_counter: i32 = 0;
    let mut has_code_copy_passed: bool = false;

    for (index, _) in bytecode.chars().enumerate() {
        if index as i32 == skip_to_index && index % 2 == 0 {
            let current_instruction: String = bytecode[index..index + 2].to_string();

            let current_instruction_size: i32 =
                opcodes::get_opcode_size(&current_instruction).unwrap() as i32;

            skip_to_index = index as i32 + current_instruction_size;

            // divide bytecode into 0 -> current_instruction + 3(instructions) and current_instruction + 4(instructions) -> end
            if current_instruction == String::from("39") || has_code_copy_passed {
                init_code_seperation_counter += 1;
                has_code_copy_passed = true;
            }

            if init_code_seperation_counter == 5 {
                let bytecode_ref = bytecode;
                let (init_bytecode, runtime_bytecode) = bytecode_ref.split_at(index);
                return Some((init_bytecode.to_string(), runtime_bytecode.to_string()));
            }
        }
    }
    return None;
}
