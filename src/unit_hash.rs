#[rustfmt::skip]
mod test {
    use std::hash::Hash;
    use std::collections::hash_map::DefaultHasher;
    fn trival() {
        let mut s = DefaultHasher::default();
        let x = ();
        //> [RS-E1017]: "Calling `.hash(_)` on expression with unit-type"
        let hashed = x.hash(&mut s);
    }
    fn resolved_unit_type() {
        type T = ();
        let mut s = DefaultHasher::default();
        let x = vec![1, 2, 3].extend([1, 2, 3]);
        //> [RS-E1017]: "Calling `.hash(_)` on expression with unit-type"
        let hashed = x.hash(&mut s);
    }
}
