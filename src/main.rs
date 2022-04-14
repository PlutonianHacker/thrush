use std::rc::Rc;

use thrush::{Thrush, value::Class};

fn main() -> Result<(), String> {
    let mut thrush = Thrush::new();

    let scope = thrush.globals();

    scope.add("x", 34);
    scope.add("y", "Hello".to_string());
    scope.add("z", true);

    assert_eq!(scope.get::<i32>("x")?, 34);
    assert_eq!(scope.get::<String>("y")?, "Hello".to_string());
    assert_eq!(scope.get::<bool>("z")?, true);

    thrush.exec("class Bird {}")?;

    let scope = thrush.globals();

    assert_eq!(scope.get::<Rc<Class>>("Bird")?.as_ref(), &Class::new("Bird"));

    thrush.exec("Bird()")?;

    //println!("{thrush:#?}");

    Ok(())
}
