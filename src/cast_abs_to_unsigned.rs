fn foo() {
    let x = 9i32;
    //> [RS-W1131]: "This cast is redundant, consider using `unsigned_abs()`"
    let y = x.abs() as u32;

    let x = -9i16;
    //> [RS-W1131]: "This cast is redundant, consider using `unsigned_abs()`"
    let y = x.abs() as u16;
}