use std::collections::HashMap;

use crate::value::{FromValue, Value};

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

    pub fn add<T: Into<Value>>(&mut self, name: &str, value: T) {
        self.globals.insert(name.into(), value.into());
    }

    pub fn get<T: FromValue>(&self, name: &str) -> Result<T, String> {
        let value = self.globals.get(name.into()).expect("cannot find name in this scope.");
        //.ok_or("cannot find name in scope".into())
        //.map(|v| T::from_value(v)?)

        Ok(T::from_value(value)?)
    }
}
