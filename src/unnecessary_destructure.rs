#[derive(Clone, Copy)]
struct S {
    s: &'static str,
}

fn foo() {
    let a = S { s: "Hello, world!" };
    //> [RS-W1130]
    let b = S { ..a };
}
