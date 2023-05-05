use std::env;

#[rustfmt::skip]
fn trivial() {
        let _ = ::std::env::var("RUSTFLAGS");
        ::std::env::set_var("RUSTFLAGS", "1");
        ::std::env::remove_var("RUSTFLAGS");

        let _ = std::env::var("RUSTFLAGS");
        std::env::set_var("RUSTFLAGS", "1");
        std::env::remove_var("RUSTFLAGS");

        let _ = env::var("RUSTFLAGS");
        env::set_var("RUSTFLAGS", "1");
        env::remove_var("RUSTFLAGS");
}

#[rustfmt::skip]
fn no_match() {
    static RUSTFLAGS: &str = "RUSTFLAGS";
    let _ = std::env::var(RUSTFLAGS);
    std::env::set_var(RUSTFLAGS, "-1");
}
