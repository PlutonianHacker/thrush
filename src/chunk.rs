use crate::{instruction::Instruction, value::{Callable}};

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

pub struct Module {
    pub functions: Vec<Box<dyn Callable>>,
}
