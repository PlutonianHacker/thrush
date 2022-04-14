use std::rc::Rc;

use crate::{
    chunk::Chunk,
    hash::Hash,
    instruction::{InstanceValue, Instruction},
    scope::State,
    value::{Value, Class},
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

    /// Reset the stack.
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}

#[derive(Debug)]
pub struct VmError(pub String);

/// The Thrush stack-based virtual machine.
#[derive(Debug)]
pub struct Vm {
    /// The operand stack.
    stack: Stack,
    /// Track the VM's global state.
    pub state: State,
    /// A chunk of bytecode.
    chunk: Rc<Chunk>,
    /// index pointer
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            state: State::new(),
            stack: Stack::new(),
            chunk: Rc::new(Chunk::new()),
            ip: 0,
        }
    }

    /// Reset the VM's internal state.
    pub fn reset(&mut self) {
        self.stack.clear();
        self.ip = 0;
    }

    pub fn execute(&mut self, chunk: Rc<Chunk>) -> Result<(), VmError> {
        self.chunk = chunk;
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

    fn op_get_prop(&mut self, _name: Hash) -> Result<(), VmError> {
        let _instance = self.stack.pop()?;

        Ok(())
    }

    fn op_call(&mut self) -> Result<(), VmError> {
        match self.stack.pop()? {
            Value::Class(class) => {
                let instance = Class::instance(class);

                self.stack.push(Value::Instance(instance));

                Ok(())
            }
            value => Err(VmError(format!("'{value}' is not callable")))
        }
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
                Instruction::CallInstance { .. } => {}
                Instruction::Call => self.op_call()?,
                Instruction::GetProperty { name } => self.op_get_prop(name)?,
                Instruction::GetGlobal { index } => {
                    let name = &*self.chunk.variables[index];

                    let value = self.state.get::<Value>(name).map_err(|e| VmError(e))?;

                    println!("{value}"); 

                    self.stack.push(value);
                }
                Instruction::Halt => break,
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    //use std::rc::Rc;

    use crate::{compiler::Compiler, lexer::Lexer, parser::Parser, scope::State};

    //use super::Vm;

    #[test]
    fn test_vm() {
        let ast = Parser::new(Lexer::tokenize("1 + 2")).parse().unwrap();
        let mut scope = State::new();

        let mut compiler = Compiler::new(&mut scope);
        let _chunk = compiler.run(ast).unwrap();

        //let mut vm = Vm::new(Rc::new(chunk));

        //vm.execute().unwrap();
    }
}
