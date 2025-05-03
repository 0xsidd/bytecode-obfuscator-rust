use std::fmt::format;

use crate::constant::opcodes;
use rand::Rng;

// function to append JUMP dest at the end of the bytecode

pub fn append_jumpdest(bytecode: &mut String) -> &mut String {
    let jumpdest_bytecode = String::from("5b");
    bytecode.push_str(&jumpdest_bytecode);
    return bytecode;
}

pub fn append_push_jump<'a>(bytecode: &'a mut String, jump_to: String) -> &'a mut String {
    let mut jump_to_padded: String = jump_to;
    if jump_to_padded.len() == 1 {
        jump_to_padded = format!("000{}", jump_to_padded);
    } else if jump_to_padded.len() == 2 {
        jump_to_padded = format!("00{}", jump_to_padded);
    } else if jump_to_padded.len() == 3 {
        jump_to_padded = format!("0{}", jump_to_padded);
    }
    let push_jump_bytecode: String = String::from("61") + &jump_to_padded + &String::from("56");
    bytecode.push_str(&push_jump_bytecode);
    return bytecode;
}

// function to get total instructions count in the bytecode
pub fn get_instruction_count(bytecode: &String) -> i32 {
    let mut skip_to_index: i32 = 0;
    let mut instruction_position: i32 = 0;

    for (index, _) in bytecode.chars().enumerate() {
        if index as i32 == skip_to_index && index % 2 == 0 {
            let current_instruction = bytecode[index..index + 2].to_string();

            let current_instruction_size: i32 =
                opcodes::get_opcode_size(&current_instruction).unwrap() as i32;

            skip_to_index = index as i32 + current_instruction_size;

            instruction_position += 1;
        }
    }

    return instruction_position - 1;
}

pub fn get_appended_jumpdest_pos(bytecode: &String) -> i32 {
    let mut skip_to_index: i32 = 0;
    let mut instruction_position: i32 = 0;

    for (index, _) in bytecode.chars().enumerate() {
        if index as i32 == skip_to_index && index % 2 == 0 {
            let current_instruction = bytecode[index..index + 2].to_string();

            let current_instruction_size: i32 =
                opcodes::get_opcode_size(&current_instruction).unwrap() as i32;

            skip_to_index = index as i32 + current_instruction_size;

            instruction_position += current_instruction_size / 2;
        }
    }

    return instruction_position - 1;
}

// function to return a random dead bytecode
pub fn get_random_dead_bytecode() -> String {
    let dead_bytecodes: Vec<String> = vec![
        String::from(
            "5b61000b566005600601505b603260331650603460351750603660371850600060011460ff57",
        ),
        String::from(
            "5b61000b566001600201505b6002600301506100036004025060006001146100fa5760056006035060078001506008800250600960010350",
        ),
        String::from(
            "5b61000b566001600201505b61000360040250600060011460fa57600160021060fb57600260031460fc57600560061650",
        ),
        String::from(
            "5b61000b566003600401505b6005600660078190035061000860090250600a600b1060fd57600c600d1060fe5760018001505050",
        ),
        String::from(
            "5b61000b566002600402505b6200000a60020150600160030a5061000460020650600060011460fe57600260031850600450",
        ),
        String::from(
            "5b61000b566001600201505b60036004600508506002600360040950610006600760081060fd576001600060021260fe5760098002035050",
        ),
        String::from(
            "5b61000b566005600601505b600760080150600a600903600060011460f25760028002506003905050",
        ),
        String::from(
            "5b61000b566005600601505b600b600c0250600d600e0450600160021060f357600360041060f457600560061650",
        ),
        String::from(
            "5b61000b566005600601505b600f60101650601160121750601360141850601560160650600060011460f657",
        ),
        String::from(
            "5b61000b566005600601505b6017601801506019601a0250601b601c0350601d601e0450600060011460f757",
        ),
    ];
    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(1..=dead_bytecodes.len() as u32 - 1); // inclusive range 1–100
    return dead_bytecodes[random_number as usize].clone();
}

// function to modify push value at a particular index
pub fn modify_push_val<'a>(
    bytecode: &'a mut String,
    push_byte_offset: i32,
    replacement_value: &i32,
) -> &'a mut String {
    let offset = push_byte_offset as usize;
    let opcode = bytecode[offset..offset + 2].to_string();
    let opcode_size = opcodes::get_opcode_size(&opcode).unwrap() as i32;
    let replacement_val_hex = format!("{:x}", replacement_value);

    println!("Replacement value in hex is {}", replacement_val_hex);

    bytecode.replace_range(
        offset + 2..offset + opcode_size as usize,
        &replacement_val_hex,
    );

    return bytecode;
}
