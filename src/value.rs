#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
}

pub trait FromValue: Sized {
    fn from_value(value: &Value) -> Result<Self, String>;
}

impl From<Value> for f32 {
    fn from(value: Value) -> f32 {
        match value {
            Value::Float(v) => v as f32,
            _ => unreachable!(),
        }
    }
}

impl From<Value> for String {
    fn from(value: Value) -> String {
        match value {
            Value::String(s) => s,
            _ => unreachable!(),
        }
    }
}

macro_rules! impl_into_value {
    ($typ:ty, $T:ident, $as: ty) => {
        impl Into<Value> for $typ {
            fn into(self) -> Value {
                Value::$T(self as $as)
            }
        }
    };
    ($typ:ty, $T:ident) => {
        impl Into<Value> for $typ {
            fn into(self) -> Value {
                Value::$T(self.into())
            }
        }
    };
}

impl_into_value!(&str, String);
impl_into_value!(String, String);
impl_into_value!(&String, String);
impl_into_value!(Box<str>, String);
impl_into_value!(&mut str, String);
impl_into_value!(i64, Integer);
impl_into_value!(i8, Integer, i64);
impl_into_value!(i16, Integer, i64);
impl_into_value!(i32, Integer, i64);
impl_into_value!(i128, Integer, i64);
impl_into_value!(isize, Integer, i64);
impl_into_value!(u8, Integer, i64);
impl_into_value!(u16, Integer, i64);
impl_into_value!(u32, Integer, i64);
impl_into_value!(u64, Integer, i64);
impl_into_value!(u128, Integer, i64);
impl_into_value!(usize, Integer, i64);
impl_into_value!(f64, Float);
impl_into_value!(f32, Float, f64);
impl_into_value!(bool, Bool);

macro_rules! impl_from_value {
    ($typ:ty, ($T:pat => $e:expr)) => {
        impl FromValue for $typ {
            fn from_value(value: &Value) -> Result<$typ, String> {
                match value {
                    $T => $e,
                    _ => Err("cannot coerce type from value".into()),
                }
            }
        }
    };
}

impl_from_value!(f64, (Value::Float(v) => Ok(*v)));
impl_from_value!(f32, (Value::Float(v) => Ok(*v as f32)));
impl_from_value!(i32, (Value::Integer(v) => Ok(*v as i32)));
impl_from_value!(i64, (Value::Integer(v) => Ok(*v as i64)));
impl_from_value!(String, (Value::String(v) => Ok(v.to_string())));
impl_from_value!(bool, (Value::Bool(v) => Ok(*v)));

#[cfg(test)]
pub mod test {
    #[test]
    fn test_value() {}
}
