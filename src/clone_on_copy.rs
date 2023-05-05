fn foo() {
    let a : i32 = 8;
    //> [RS-W1110]: "Call to `clone()` on type that implements Copy"
    let b = a.clone();
    //> [RS-W1110]: "Call to `clone()` on type that implements Copy"
    let c = (&a).clone();
    let d = String::from("Hi there");
    let dr = &d;
    let e = d.clone();
    let f = dr.clone(); // don't raise if it's a reference to non Copy type
}
