#![rustfmt::skip]
fn foo() {
    let f: fn() = unsafe { std::mem::transmute(std::ptr::null::<fn()>()) };
    if (f as *const ()).is_null() {}
}
