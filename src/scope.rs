use std::{collections::HashMap, rc::Rc};

use crate::value::{FromValue, Value, ToValue, Class};

#[derive(Debug)]
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
        self.globals.insert(name.into(), Value::Class(Rc::new(Class::new(name.into()))));
    }

    pub fn get<T: FromValue>(&self, name: &str) -> Result<T, String> {
        let value = self.globals.get(name.into()).expect("cannot find name in this scope.");

        Ok(T::from_value(value)?)
    }
}
