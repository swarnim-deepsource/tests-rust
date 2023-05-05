#[rustfmt::skip]
mod test {
    use std::collections::hash_map::DefaultHasher;
    fn foo() {
        //> [RS-W1079]: "Empty call to `new()`"
        let a = String::new();
        //> [RS-W1079]: "Empty call to `new()`"
        let mut h = DefaultHasher::new();
    }
}
