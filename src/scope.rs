use std::{collections::HashMap};

use crate::value::{Class, FromValue, ToValue, Value};

/// Struct for tracking global state.
#[derive(Debug, Default)]
pub struct State {
    globals: HashMap<String, Value>,
}

impl State {
    pub fn new() -> Self {
        State {
            globals: HashMap::new(),
        }
    }

    pub fn add<T: ToValue>(&mut self, name: &str, value: T) {
        self.globals.insert(name.into(), value.to_value());
    }

    pub fn add_class<S: Into<String> + Copy>(&mut self, name: S) {
        self.globals
            .insert(name.into(), Value::Class(Class::new(name.into())));
    }

    pub fn get<T: FromValue>(&self, name: &str) -> Result<T, String> {
        let value = self
            .globals
            .get(name)
            .expect("cannot find name in this scope.");

        T::from_value(value)
    }
}
