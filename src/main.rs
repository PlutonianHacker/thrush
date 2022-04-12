use thrush::Thrush;

fn main() {
    let mut thrush = Thrush::new();

    let scope = thrush.globals();

    scope.add("x", 34);
    scope.add("y", "Hello".to_string());
    scope.add("z", true);

    assert_eq!(scope.get::<i32>("x"), Ok(34));
    assert_eq!(scope.get::<String>("y"), Ok("Hello".to_string()));
    assert_eq!(scope.get::<bool>("z"), Ok(true));

    println!("{thrush:#?}");
}
