#[rustfmt::skip]
use std::rc::Rc;

fn foo() {
    let mut v = Vec::new();
    v.push(Rc::new(0));
    let r = Rc::new(1);
    //> [RS-W1106]: "Insertion of pointer to non-owned reference counted value into vector"
    v.push(r.clone());
    //> [RS-W1106]: "Insertion of pointer to non-owned reference counted value into vector"
    v.insert(0, r.clone());
}
