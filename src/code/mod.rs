mod tests;

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

fn make_instruction(op: OpCode, operands: Vec<u32>) -> Vec<u8> {
    let operand_widths = op.lookup().1;
    let mut instruction: Vec<u8> = Vec::new();
    if operands.len() != operand_widths.len() {
        instruction
    } else {
        instruction.push(op as u8);

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
