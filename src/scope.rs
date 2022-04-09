use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug)]
pub struct Scope {
    globals: HashMap<String, Value>
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            globals: HashMap::new(),
        }
    }

    pub fn add<T: Into<Value>>(&mut self, name: &str, value: T) {
        self.globals.insert(name.into(), value.into());
    }
    
    pub fn get<T: From<Value>>(&self, name: &str) -> T {
        T::from(self.globals.get(name.into()).expect("cannot find name in scope").clone())
    } 
}