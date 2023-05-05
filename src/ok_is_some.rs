#[rustfmt::skip]
mod test {

    fn trivial() {
        let x: Result<u32, u32> = Ok(5);
        //> [RS-W1055]: "Called `.ok().is_some()` on a `Result` value, call `.is_ok()` directly instead"
        x.ok().is_some();
        //> [RS-W1055]
        x.ok().is_none();
        //> [RS-W1055]
        x.err().is_some();
        //> [RS-W1055]
        x.err().is_none();
    }

    fn fake_result() {
        struct X;

        impl X {
            fn ok(&self) -> Self { unimplemented!() }
            fn is_some(&self, _: &str) { unimplemented!() }
        }

        let x = X;
        x.ok().is_some("hello");
    }


}
