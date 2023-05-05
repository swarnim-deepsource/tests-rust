enum A {
    B(i32),
    C,
}

fn foo() {
    let _ = A::B as usize;
}

#[repr(C)]
enum D {
    E = 1,
    F = 2,
}

fn bar() {
    // This works
    let _ = D::E as usize;
}
