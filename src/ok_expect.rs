#[rustfmt::skip]
mod test {
    fn trivial() {
        let x: Result<u32, u32> = Ok(5);
                x.ok().expect("hello");
                x.ok().unwrap();
    }

    fn fake_result() {
        struct X;

        impl X {
            fn ok(&self) -> Self { unimplemented!() }
            fn expect(&self, _: &str) { unimplemented!() }
        }

        let x = X;
        x.ok().expect("hello");
    }

    fn no_debug_impl() {
        struct X;

        let x: Result<u32, X> = Err(X);
        x.ok().expect("hello");
    }
}
