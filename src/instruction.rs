use crate::value::Value;

/// An instruction in a stack-based virtual machine.
///
/// Each instruction is composed of an opcode
/// followed by as many as three arguments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    /// Push a value onto the stack.
    Push { value: InstanceValue },
    /// Pop a value off the stack.
    Pop,
    /// Construct a new class.
    Class { index: usize },
    /// Call the value on top of the stack.
    Call,
    /// Load a `nil` value onto the stack.
    LoadNil,
    /// Access a property from the instance on top of the stack.
    GetProperty { index: usize },
    /// Define a new global.
    DefineGlobal { index: usize },
    /// Set a global's value to what's on top of the stack.
    SetGlobal { index: usize },
    /// Load a global onto the stack.
    GetGlobal { index: usize },
    /// Halt the current VM.
    Halt,
}

impl Instruction {
    pub fn integer(v: i64) -> Self {
        Self::Push {
            value: InstanceValue::Integer(v),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstanceValue {
    Bool(bool),
    Integer(i64),
    Float(f64),
}

impl InstanceValue {
    pub fn into_value(&self) -> Value {
        match *self {
            Self::Bool(b) => Value::Bool(b),
            Self::Integer(i) => Value::Integer(i),
            Self::Float(f) => Value::Float(f),
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    use super::Instruction;

    #[test]
    fn test_instruction_size() {
        assert!(mem::size_of::<Instruction>() <= 24);
    }
}
