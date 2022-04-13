use std::collections::HashMap;

use crate::value::{FromValue, Value, ToValue};

#[derive(Debug)]
pub struct Scope {
    globals: HashMap<String, Value>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            globals: HashMap::new(),
        }
    }

    pub fn add<T: ToValue>(&mut self, name: &str, value: T) {
        self.globals.insert(name.into(), value.to_value());
    }

    pub fn get<T: FromValue>(&self, name: &str) -> Result<T, String> {
        let value = self.globals.get(name.into()).expect("cannot find name in this scope.");

        Ok(T::from_value(value)?)
    }
}
