/* disables = ["default-outside-trait"] */
struct B;

impl B {
    fn default() -> Self {
        Self
    }
}

fn foo() {
    let a = Box::new(String::default());
    let b = Box::new(B::default());
}
