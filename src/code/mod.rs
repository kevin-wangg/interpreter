use std::fmt::Display;

#[repr(u8)]
pub enum OpCode {
    OpConstant,
}

impl OpCode {
    // Returns a human readable name for the opcode and a vector of widths of the operands
    fn lookup(&self) -> (&str, Vec<u32>) {
        match self {
            OpCode::OpConstant => ("OpConstant", vec![2]),
        }
    }
}

pub fn make_instruction(opcode: OpCode, operands: Vec<u32>) -> Vec<u8> {
    let operand_widths = opcode.lookup().1;
    let mut instruction: Vec<u8> = Vec::new();
    if operands.len() != operand_widths.len() {
        instruction
    } else {
        instruction.push(opcode as u8);

        for i in 0..operands.len() {
            match operand_widths[i] {
                2 => {
                    let operand: u16 = operands[i]
                        .try_into()
                        .expect("Operand too large for 2 byte width");
                    instruction.extend_from_slice(&operand.to_be_bytes());
                }
                _ => {}
            };
        }

        instruction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make() {
        let tests = vec![(
            OpCode::OpConstant,
            vec![65534],
            vec![OpCode::OpConstant as u8, 255, 254],
        )];
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
}
