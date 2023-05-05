#[rustfmt::skip]
fn f() {
    let x = vec![1u32];
    let y = &&x;
        let z = y.clone();
}
