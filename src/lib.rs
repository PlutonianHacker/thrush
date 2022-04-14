//! The Thrush scripting language, including the compiler and VM.

pub mod ast;
pub mod chunk;
pub mod compiler;
pub mod instruction;
pub mod lexer;
pub mod parser;
pub mod scope;
pub mod token;
pub mod value;
pub mod vm;
pub mod hash;

use std::rc::Rc;

use compiler::Compiler;
use scope::State;
use vm::Vm;

#[derive(Debug)]
pub struct Thrush {
    vm: Vm,
}

impl Thrush {
    pub fn new() -> Self {
        Thrush {
            vm: Vm::new(),
        }
    }

    /// Get a mutable reference to Thrush's global state.
    pub fn globals(&mut self) -> &mut State {
        &mut self.vm.state
    }

    pub fn exec(&mut self, script: &str) -> Result<(), String> {
        self._exec(script)
    } 

    fn _exec(&mut self, script: &str) -> Result<(), String> {
        let tokens = lexer::Lexer::tokenize(script);
        let ast = parser::Parser::parse_ast(tokens)?; 
        let mut compiler = Compiler::new(&mut self.vm.state);

        let chunk = compiler.run(ast)?;

        self.vm.execute(Rc::new(chunk)).unwrap();

        self.vm.reset();

        Ok(())
    }

    /// Get a mutable reference to the Thrush's vm.
    pub fn vm_mut(&mut self) -> &mut Vm {
        &mut self.vm
    }
}

impl Default for Thrush {
    fn default() -> Self {
        Self::new()
    }
}
