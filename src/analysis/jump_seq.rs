use crate::constant::opcodes;

#[derive(Debug)]
pub struct Push_Positions {
    byteoffset_decimal: i32,
    byteoffset_hex: String,
    value_decimal: i32,
    value_hex: String,
    next_instruction_code: String,
}

pub fn find_jump_seq(bytecode: &String) -> Vec<Push_Positions> {
    let mut current_instruction: String = String::from("00");
    let mut next_instruction: String = String::from("00");

    let mut skip_to_index: i32 = 0;

    let mut jump_sequences: Vec<Push_Positions> = Vec::new();

    for (index, _) in bytecode.chars().enumerate() {
        if index as i32 == skip_to_index && index % 2 == 0 {
            current_instruction = bytecode[index..index + 2].to_string();

            let current_instruction_size =
                opcodes::get_opcode_size(&current_instruction).unwrap() as i32;

            skip_to_index = index as i32 + current_instruction_size;

            let current_params = bytecode[index + 2..skip_to_index as usize].to_string();

            if skip_to_index as usize + 2 <= bytecode.len() {
                next_instruction =
                    bytecode[skip_to_index as usize..skip_to_index as usize + 2].to_string();
            }

            let push_jmp_seq: Option<Push_Positions> = check_push_jump_seq(
                &current_instruction,
                &next_instruction,
                &index,
                &current_params,
            );

            match push_jmp_seq {
                Some(val) => jump_sequences.push(val),
                None => {}
            }
        }
    }

    return jump_sequences;
}

fn check_push_jump_seq(
    current_instruction: &String,
    next_instruction: &String,
    push_index: &usize,
    push_value: &String,
) -> Option<Push_Positions> {
    if next_instruction == &String::from("56")
        && (current_instruction.starts_with("6") || current_instruction.starts_with('7'))
    {
        let (value_decimal, value_hex) = if push_value == "" {
            (0, "00".to_string())
        } else {
            (
                i32::from_str_radix(push_value, 16).unwrap(),
                push_value.clone(),
            )
        };
        let push_seq = Push_Positions {
            byteoffset_decimal: *push_index as i32,
            byteoffset_hex: format!("{:x}", push_index),
            value_decimal,
            value_hex,
            next_instruction_code: current_instruction.clone(),
        };
        Some(push_seq)
    } else {
        None
    }
}
