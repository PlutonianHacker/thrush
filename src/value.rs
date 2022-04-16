use core::fmt;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt::Debug,
    rc::Rc,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Integer(i64),
    String(String),
    Instance(Rc<Instance>),
    Class(Rc<Class>),
    Method(Rc<BoundMethod>),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(v) => f.write_fmt(format_args!("{v}")),
            Value::Float(v) => f.write_fmt(format_args!("{v}")),
            Value::Integer(v) => f.write_fmt(format_args!("{v}")),
            Value::String(v) => f.write_fmt(format_args!("{v}")),
            Value::Instance(instance) => f.write_fmt(format_args!("{instance}")),
            Value::Class(class) => f.write_fmt(format_args!("<Class {}>", class.as_ref().name)),
            Value::Method(method) => f.write_fmt(format_args!(
                "<method {}.{}>",
                method.receiver.as_ref().class.as_ref().name,
                method.function.name
            )),
            Value::Nil => f.write_str("nil"),
        }
    }
}

/// Representation of a Thrush class in rust.
pub struct Class {
    pub name: Box<str>,
    pub methods: RefCell<HashMap<Box<str>, Rc<InstanceFun>>>,
}

impl Class {
    pub fn new<S: Into<Box<str>>>(name: S) -> Rc<Self> {
        Rc::new(Self {
            name: name.into(),
            methods: RefCell::new(HashMap::new()),
        })
    }

    pub fn add_method<S: Into<Box<str>> + Copy>(
        &self,
        name: S,
        fun: fn(Rc<Instance>, Vec<Value>) -> Value,
    ) {
        self.methods
            .borrow_mut()
            .insert(name.into(), Rc::new(InstanceFun::new(name.into(), fun)));
    }

    pub fn instance(self: Rc<Self>) -> Rc<Instance> {
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
    fn call(&self, args: Vec<Value>) -> Value {
        (self.inner)(args)
    }
}

/// An instance of a [Class].
#[derive(Debug, PartialEq)]
pub struct Instance {
    pub class: Rc<Class>,
    pub fields: RefCell<Vec<Value>>,
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Rc<Self> {
        Rc::new(Self {
            class,
            fields: RefCell::new(Vec::new()),
        })
    }

    /// Bind a method with the given name and call it immediately.
    pub fn invoke<S: Into<Box<str>>>(receiver: Rc<Self>, name: S) -> Value {
        let bound = Instance::bind(receiver, name);

        bound.call(vec![])
    }

    /// Bind a method to an instance.
    pub fn bind<S: Into<Box<str>>>(receiver: Rc<Self>, name: S) -> BoundMethod {
        let instance = receiver.as_ref().class.as_ref().methods.borrow();
        let method = instance.get(&name.into()).unwrap();

        BoundMethod::new(receiver.clone(), method.clone())
    }

    /// Get a mutable reference to the instance's fields.
    pub fn fields_mut(&self) -> RefMut<'_, Vec<Value>> {
        self.fields.borrow_mut()
    }

    /// Get a reference to the instance's fields.
    pub fn fields(&self) -> Ref<'_, Vec<Value>> {
        self.fields.borrow()
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("<instance {}>", self.class.as_ref().name))
    }
}

#[derive(Debug)]
pub struct InstanceFun {
    pub name: Box<str>,
    pub fun: fn(Rc<Instance>, Vec<Value>) -> Value,
}

impl InstanceFun {
    pub fn new<S: Into<Box<str>>>(name: S, fun: fn(Rc<Instance>, Vec<Value>) -> Value) -> Self {
        Self {
            name: name.into(),
            fun,
        }
    }
}

impl PartialEq for InstanceFun {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, PartialEq)]
pub struct BoundMethod {
    pub receiver: Rc<Instance>,
    pub function: Rc<InstanceFun>,
}

impl<'a, 'b> BoundMethod {
    pub fn new(receiver: Rc<Instance>, function: Rc<InstanceFun>) -> Self {
        Self { receiver, function }
    }
}

impl Callable for BoundMethod {
    fn call(&self, args: Vec<Value>) -> Value {
        (self.function.fun)(self.receiver.clone(), args)
    }
}

pub trait Callable {
    fn call(&self, args: Vec<Value>) -> Value;
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

impl From<()> for Value {
    fn from(_: ()) -> Value {
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
impl_from_value!(Rc<Class>, (Value::Class(v) => Ok(v.to_owned())));

impl FromValue for Value {
    fn from_value(value: &Value) -> Result<Self, String> {
        Ok(value.to_owned())
    }
}

#[cfg(test)]
pub mod test {
    use std::{mem, rc::Rc};

    use super::{BoundMethod, Callable, Class, FromValue, Instance, InstanceFun, ToValue, Value};

    #[test]
    fn test_bound_method() {
        let class = Class::new("Test");
        let receiver = class.instance();

        let fun1 = InstanceFun::new("x", |this, _| {
            this.fields_mut().push(Value::Integer(10));
            Value::Nil
        });

        let fun2 = InstanceFun::new("y", |this, _| {
            if let Value::Integer(v) = &mut this.fields_mut()[0] {
                *v += 1;
            }
            Value::Nil
        });

        let mut method1 = BoundMethod::new(receiver.clone(), Rc::new(fun1));
        let mut method2 = BoundMethod::new(receiver.clone(), Rc::new(fun2));

        BoundMethod::call(&mut method1, vec![]);

        for _ in 0..10 {
            BoundMethod::call(&mut method2, vec![]);
        }

        assert_eq!(&receiver.fields_mut()[0], &Value::Integer(20));
    }

    #[test]
    fn test_bound_method_args() {
        let class = Class::new("Args");
        let receiver = class.instance();

        let constructor = InstanceFun::new("constructor", |this, _| {
            this.fields_mut().push(Value::Integer(1));
            Value::Nil
        });

        let add = InstanceFun::new("add", |this, args| {
            (i32::from_value(&this.fields()[0]).unwrap() + i32::from_value(&args[0]).unwrap())
                .to_value()
        });

        let mut constructor = BoundMethod::new(receiver.clone(), Rc::new(constructor));
        let mut method = BoundMethod::new(receiver.clone(), Rc::new(add));

        BoundMethod::call(&mut constructor, vec![]);

        assert_eq!(
            BoundMethod::call(&mut method, vec![2_i32.to_value()]),
            3_i32.to_value()
        );
    }

    #[test]
    fn test_class() {
        let class = Class::new("Io");

        class.add_method("to_string", |_, _| Value::String("__io__".into()));

        class.add_method("print", |this, args| {
            let name = Instance::invoke(this, "to_string");

            println!("{name}");
            println!("{}", args[0]);

            Value::Nil
        });

        let receiver = class.instance();

        let mut bound = Instance::bind(receiver, "print");

        BoundMethod::call(&mut bound, vec!["Hello, World!".to_value()]);
    }

    #[test]
    #[allow(dead_code)]
    fn test_value_size() {
        //assert_eq!(16, mem::size_of::<Value>())
        assert_eq!(32, mem::size_of::<Value>())
    }
}
