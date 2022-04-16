use crate::{instruction::Instruction, value::Callable};

#[derive(Debug, Default)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub variables: Vec<Box<str>>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn add_variable<S: Into<Box<str>>>(&mut self, s: S) -> usize {
        self.variables.push(s.into());
        self.variables.len() - 1
    }
}

pub struct Module {
    pub functions: Vec<Box<dyn Callable>>,
}
