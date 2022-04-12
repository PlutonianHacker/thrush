use crate::instruction::Instruction;

#[derive(Debug)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::default(),
        }
    }
}
