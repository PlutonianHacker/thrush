use core::fmt;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
    Instance(Rc<RefCell<Instance>>),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(v) => f.write_fmt(format_args!("{v}")),
            Value::Float(v) => f.write_fmt(format_args!("{v}")),
            Value::Integer(v) => f.write_fmt(format_args!("{v}")),
            Value::String(v) => f.write_fmt(format_args!("{v}")),
            Value::Nil => f.write_str("nil"),
            Value::Instance(instance) => f.write_fmt(format_args!(
                "<instance {:?}>",
                instance.as_ref().borrow().state
            )),
        }
    }
}

pub struct Function {
    pub name: Box<str>,
    pub inner: fn(Vec<Value>) -> Value,
}

impl Function {
    pub fn new<T: Into<Box<str>>>(name: T, inner: fn(Vec<Value>) -> Value) -> Self {
        Self {
            name: name.into(),
            inner,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instance {
    pub state: Vec<Value>,
}

impl Instance {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(Self { state: Vec::new() }.into())
    }
}

pub struct InstanceFun<I> {
    pub name: Box<str>,
    pub fun: fn(RefMut<Instance>, Vec<Value>) -> I,
}

impl<I> InstanceFun<I> {
    pub fn new<T: Into<Box<str>>>(name: T, fun: fn(RefMut<Instance>, Vec<Value>) -> I) -> Self {
        Self {
            name: name.into(),
            fun,
        }
    }
}

pub struct BoundMethod<'a, I> {
    pub receiver: Rc<RefCell<Instance>>,
    pub function: &'a InstanceFun<I>,
}

impl<'a, I> BoundMethod<'a, I> {
    pub fn new(receiver: Rc<RefCell<Instance>>, function: &'a InstanceFun<I>) -> Self {
        Self { receiver, function }
    }
}

impl<I: Into<Value>> Callable for BoundMethod<'_, I> {
    fn call(slf: &mut Self, args: Vec<Value>) -> Value {
        let this = slf.receiver.as_ref().borrow_mut();
        (slf.function.fun)(this, args).into()
    }
}

pub trait Callable {
    fn call(slf: &mut Self, args: Vec<Value>) -> Value;
}

pub trait FromValue: Sized {
    fn from_value(value: &Value) -> Result<Self, String>;
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

impl Into<Value> for () {
    fn into(self) -> Value {
        Value::Nil
    }
}

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
    use crate::value::{FromValue, Instance, InstanceFun};

    use super::{BoundMethod, Callable, Value};

    #[test]
    fn test_bound_method() {
        let receiver = Instance::new();

        let fun1 = InstanceFun::new("x", |mut this, _| {
            this.state.push(Value::Integer(10));
        });

        let fun2 = InstanceFun::new("y", |mut this, _| {
            if let Value::Integer(v) = &mut this.state[0] {
                *v += 1;
            }
        });

        let mut method1 = BoundMethod::new(receiver.clone(), &fun1);
        let mut method2 = BoundMethod::new(receiver.clone(), &fun2);

        BoundMethod::call(&mut method1, vec![]);

        for _ in 0..10 {
            BoundMethod::call(&mut method2, vec![]);
        }

        assert_eq!(&receiver.as_ref().borrow().state[0], &Value::Integer(20));
    }

    #[test]
    fn test_bound_method_args() {
        let receiver = Instance::new();

        let constructor = InstanceFun::new("constructor", |mut this, _| {
            this.state.push(Value::Integer(1));
        });

        let add = InstanceFun::new("add", |this, args| {
            i32::from_value(&this.state[0]).unwrap() + i32::from_value(&args[0]).unwrap()
        });

        let mut constructor = BoundMethod::new(receiver.clone(), &constructor);
        let mut method = BoundMethod::new(receiver.clone(), &add);

        BoundMethod::call(&mut constructor, vec![]);

        assert_eq!(BoundMethod::call(&mut method, vec![2.into()]), 3.into());
    }
}
