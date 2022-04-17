use std::{rc::Rc};

use thrush::{value::{Class, Value}, Thrush};

fn main() -> Result<(), String> {
    let mut thrush = Thrush::new();

    let scope = thrush.globals();

    scope.add("x", 34);
    scope.add("y", "Hello".to_string());
    scope.add("z", true);

    assert_eq!(scope.get::<i32>("x")?, 34);
    assert_eq!(scope.get::<String>("y")?, "Hello".to_string());
    assert!(scope.get::<bool>("z")?);

    thrush.exec("class Bird {}")?;

    let scope = thrush.globals();

    assert_eq!(
        scope.get::<Rc<Class>>("Bird")?,
        Class::new("Bird")
    );

    let class = scope.get::<Rc<Class>>("Bird")?;

    class.add_method("sound", |_, _| {
        println!("Hello, World!");

        Value::Nil
    });

    thrush.exec("var instance = Bird()")?;
    thrush.exec("instance.sound()")?;

    // println!("{thrush:#?}");

    Ok(())
}
