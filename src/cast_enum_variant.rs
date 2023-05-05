//> scatr-check: RS-W1103
enum A {
    B(i32),
    C,
}

fn foo() {
    // Should not catch this as B is a constructor
    let _ = A::B as usize;
    // "ARCHIVE IT" [RS-W1103]: "Cast from enum variant to type"
    // let _ = A::C as usize;
    // "ARCHIVE IT" [RS-W1103]: "Cast from enum variant to type"
    // let _ = A::C as usize;
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
