#[rustfmt::skip]
fn f() {
    let x = vec![1u32];
    let y = &&x;
    //> [RS-W1100]: "Calling `.clone()` on a double reference"
    let z = y.clone();
}
