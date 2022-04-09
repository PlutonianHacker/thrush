//! The Thrush scripting language, including the compiler and VM.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod scope;
pub mod token;
pub mod value;
pub mod vm;

use scope::Scope;

#[derive(Debug)]
pub struct Thrush {
    state: Scope,
}

impl Thrush {
    pub fn new() -> Self {
        Thrush {
            state: Scope::new(),
        }
    }

    pub fn globals(&mut self) -> &mut Scope {
        &mut self.state
    }
}
