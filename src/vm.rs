use std::rc::Rc;

use crate::{
    chunk::Chunk,
    instruction::{InstanceValue, Instruction},
    value::Value,
};

/// The VM's stack.
#[derive(Debug)]
pub struct Stack {
    stack: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    /// Pop a value off the stack and return it.
    pub fn pop(&mut self) -> Result<Value, VmError> {
        self.stack
            .pop()
            .ok_or_else(|| VmError("stack should not be empty".into()))
    }

    /// Push a value onto the stack.
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
}

#[derive(Debug)]
pub struct VmError(pub String);

/// The Thrush stack-based virtual machine.
pub struct Vm {
    /// The operand stack.
    stack: Stack,
    chunk: Rc<Chunk>,
    ip: usize,
}

impl Vm {
    pub fn new(chunk: Rc<Chunk>) -> Self {
        Vm {
            stack: Stack::new(),
            chunk,
            ip: 0,
        }
    }

    pub fn execute(&mut self) -> Result<(), VmError> {
        self.run()
    }

    #[cfg_attr(feature = "bench", inline(never))]
    pub fn get_next_inst(&mut self) -> &Instruction {
        self.ip = self.ip.wrapping_add(1);
        &self.chunk.instructions[self.ip - 1]
    }

    fn op_push(&mut self, value: InstanceValue) {
        self.stack.push(value.into_value());
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        loop {
            let inst = *self.get_next_inst();
            println!("{:?}", self.stack);
            match inst {
                Instruction::Push { value } => {
                    self.op_push(value);
                }
                Instruction::Pop => {
                    self.stack.pop()?;
                }
                Instruction::CallInstance { .. } => {

                }
                Instruction::Halt => break,
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{parser::Parser, lexer::Lexer, compiler::Compiler};

    use super::Vm;

    #[test]
    fn test_vm() {
        let ast = Parser::new(Lexer::tokenize("1 + 2")).parse().unwrap();

        let mut compiler = Compiler::new();
        let chunk = compiler.run(ast).unwrap();

        let mut vm = Vm::new(Rc::new(chunk));

        vm.execute().unwrap();
    }
}
