use thrush::Thrush;

fn main() {
    let mut thrush = Thrush::new();

    let scope = thrush.globals();

    scope.add("x", 34);
    scope.add("y", "Hello");

    assert_eq!(scope.get::<f32>("x"), 34.0);
    assert_eq!(scope.get::<String>("y"), "Hello");

    println!("{thrush:#?}");
}
