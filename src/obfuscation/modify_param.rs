use crate::constant::opcodes;

pub fn modify_push_val(bytecode: &mut String, push_byte_offset: i32) -> &mut String {
    let offset = push_byte_offset as usize;
    let opcode = bytecode[offset..offset + 2].to_string();
    let opcode_size = opcodes::get_opcode_size(&opcode).unwrap() as i32;

    bytecode.replace_range(offset + 2..offset + opcode_size as usize, "0b");

    return bytecode;
}
