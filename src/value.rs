use core::fmt;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
    Instance(Rc<RefCell<Instance<'static>>>),
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
            Value::Instance(instance) => {
                f.write_fmt(format_args!("{}", instance.as_ref().borrow()))
            }
        }
    }
}

pub struct Class {
    pub name: Box<str>,
    pub methods: HashMap<Box<str>, InstanceFun>,
}

impl Class {
    pub fn new<S: Into<Box<str>>>(name: S) -> Self {
        Self {
            name: name.into(),
            methods: HashMap::new(),
        }
    }

    pub fn add_method<S: Into<Box<str>> + Copy>(
        &mut self,
        name: S,
        fun: fn(Rc<RefCell<Instance>>, Vec<Value>) -> Value,
    ) {
        self.methods
            .insert(name.into(), InstanceFun::new(name.into(), fun));
    }

    pub fn instance(&self) -> Rc<RefCell<Instance>> {
        Instance::new(self)
    }
}

impl Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Class").field("name", &self.name).finish()
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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

impl Callable for Function {
    fn call(self: &mut Self, args: Vec<Value>) -> Value {
        (self.inner)(args)
    }
}

#[derive(Debug, PartialEq)]
pub struct Instance<'a> {
    pub class: &'a Class,
    pub state: Vec<Value>,
}

impl<'a> Instance<'a> {
    pub fn new(class: &'a Class) -> Rc<RefCell<Self>> {
        Rc::new(
            Self {
                class,
                state: Vec::new(),
            }
            .into(),
        )
    }

    pub fn invoke<S: Into<Box<str>>, V: Into<Value>>(
        receiver: Rc<RefCell<Instance>>,
        name: S,
    ) -> Value {
        let mut bound = Instance::bind(receiver, name);

        BoundMethod::call(&mut bound, vec![])
    }

    pub fn bind<S: Into<Box<str>>>(receiver: Rc<RefCell<Instance>>, name: S) -> BoundMethod {
        let method = receiver
            .as_ref()
            .borrow()
            .class
            .methods
            .get(&name.into())
            .unwrap();

        BoundMethod::new(receiver.clone(), method)
    }
}

impl fmt::Display for Instance<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("<instance {}>", self.class.name))
    }
}

pub struct InstanceFun {
    pub name: Box<str>,
    pub fun: fn(Rc<RefCell<Instance>>, Vec<Value>) -> Value,
}

impl InstanceFun {
    pub fn new<S: Into<Box<str>>>(
        name: S,
        fun: fn(Rc<RefCell<Instance>>, Vec<Value>) -> Value,
    ) -> Self {
        Self {
            name: name.into(),
            fun,
        }
    }
}

pub struct BoundMethod<'a, 'b> {
    pub receiver: Rc<RefCell<Instance<'a>>>,
    pub function: &'b InstanceFun,
}

impl<'a, 'b> BoundMethod<'a, 'b> {
    pub fn new(receiver: Rc<RefCell<Instance<'a>>>, function: &'b InstanceFun) -> Self {
        Self { receiver, function }
    }
}

impl Callable for BoundMethod<'_, '_> {
    fn call(self: &mut Self, args: Vec<Value>) -> Value {
        (self.function.fun)(self.receiver.clone(), args).into()
    }
}

pub trait Callable {
    fn call(self: &mut Self, args: Vec<Value>) -> Value;
}

pub trait FromValue: Sized {
    fn from_value(value: &Value) -> Result<Self, String>;
}

pub trait ToValue {
    fn to_value(self) -> Value;
}

macro_rules! impl_into_value {
    ($typ:ty, $T:ident, $as: ty) => {
        impl ToValue for $typ {
            fn to_value(self) -> Value {
                Value::$T(self as $as)
            }
        }
    };
    ($typ:ty, $T:ident) => {
        impl ToValue for $typ {
            fn to_value(self) -> Value {
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
    //use std::{cell::RefCell, rc::Rc};

    use crate::value::{FromValue, InstanceFun, ToValue};

    use super::{BoundMethod, Callable, Class, Instance, Value};

    #[test]
    fn test_bound_method() {
        let class = Class::new("Test");
        let receiver = class.instance();

        let fun1 = InstanceFun::new("x", |this, _| {
            this.as_ref().borrow_mut().state.push(Value::Integer(10));
            Value::Nil
        });

        let fun2 = InstanceFun::new("y", |this, _| {
            if let Value::Integer(v) = &mut this.as_ref().borrow_mut().state[0] {
                *v += 1;
            }
            Value::Nil
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
        let class = Class::new("Args");
        let receiver = class.instance();

        let constructor = InstanceFun::new("constructor", |this, _| {
            this.as_ref().borrow_mut().state.push(Value::Integer(1));
            Value::Nil
        });

        let add = InstanceFun::new("add", |this, args| {
            (i32::from_value(&this.as_ref().borrow_mut().state[0]).unwrap()
                + i32::from_value(&args[0]).unwrap())
            .to_value()
        });

        let mut constructor = BoundMethod::new(receiver.clone(), &constructor);
        let mut method = BoundMethod::new(receiver.clone(), &add);

        BoundMethod::call(&mut constructor, vec![]);

        assert_eq!(
            BoundMethod::call(&mut method, vec![2_i32.to_value()]),
            3_i32.to_value()
        );
    }

    #[test]
    fn test_class() {
        let mut class = Class::new("Io");

        class.add_method("to_string", |_, _| Value::String("__io__".into()));

        class.add_method("print", |this, args| {
            let name = Instance::invoke::<&str, ()>(this, "to_string");

            println!("{name}");
            println!("{}", args[0]);

            Value::Nil
        });

        let receiver = class.instance();

        let mut bound = Instance::bind(receiver, "print");

        BoundMethod::call(&mut bound, vec!["Hello, World!".to_value()]);
    }

    #[test]
    fn test_instance_state() {
        struct U32(u32);

        let _class = Class::new("U32");

        /*class.add_method("add", |this, args| {

        });*/

        
    }
}
