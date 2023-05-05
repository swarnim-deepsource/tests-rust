#[rustfmt::skip]
mod test {

    fn trivial() {
        let x: Result<u32, u32> = Ok(5);
                x.ok().is_some();
                x.ok().is_none();
                x.err().is_some();
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
