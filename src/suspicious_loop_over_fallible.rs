#[rustfmt::skip]
fn f() {
   //> [RS-W1054]: "This `for` loops over `Option<T>`"
    for x in Option::from(10) {}

    let res: Result<i32, &str> = Ok(-23);
   //> [RS-W1054]: "This `for` loops over `Result<T, E>`"
    for x in res {}


    for x in [1] {
        /* do nothing */
    }
}