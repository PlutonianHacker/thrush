
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f32),
}

impl Into<Value> for f32 {
    fn into(self) -> Value {
        Value::Number(self)
    }
}

impl Into<Value> for isize {
    fn into(self) -> Value {
        Value::Number(self as f32)
    }
}

impl Into<Value> for &str {
    fn into(self) -> Value {
        Value::String(self.into())
    }
}

impl From<Value> for f32 {
    fn from(value: Value) -> f32 {
        match value {
            Value::Number(v) => v,
            _ => unreachable!()
        }
    }
}

impl From<Value> for String {
    fn from(value: Value) -> String {
        match value {
            Value::String(s) => s,
            _ => unreachable!()
        }
    }
}