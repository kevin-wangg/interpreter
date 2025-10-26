#[cfg(test)]
use crate::code::OpCode;
#[cfg(test)]
use crate::code::make_instruction;

#[test]
fn test_make() {
    let tests = vec![
        (OpCode::OpConstant, vec![65534], vec![OpCode::OpConstant as u8, 255, 254]),
    ];
    for (op_code, operands, expected) in tests {
        let instruction = make_instruction(op_code, operands);
        assert_eq!(instruction, expected);
    }
}

#[test]
#[should_panic(expected = "Operand too large for 2 byte width")]
fn test_make_instruction_panics_with_invalid_operand() {
    make_instruction(OpCode::OpConstant, vec![u32::MAX]);
}
