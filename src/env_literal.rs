use std::env;

#[rustfmt::skip]
fn trivial() {
    //> [RS-W1015]: "Using `::std::env::var` with a string literal"
    let _ = ::std::env::var("RUSTFLAGS");
    //> [RS-W1015]: "Using `::std::env::set_var` with a string literal"
    ::std::env::set_var("RUSTFLAGS", "1");
    //> [RS-W1015]: "Using `::std::env::remove_var` with a string literal"
    ::std::env::remove_var("RUSTFLAGS");

    //> [RS-W1015]: "Using `std::env::var` with a string literal"
    let _ = std::env::var("RUSTFLAGS");
    //> [RS-W1015]: "Using `std::env::set_var` with a string literal"
    std::env::set_var("RUSTFLAGS", "1");
    //> [RS-W1015]: "Using `std::env::remove_var` with a string literal"
    std::env::remove_var("RUSTFLAGS");

    //> [RS-W1015]: "Using `env::var` with a string literal"
    let _ = env::var("RUSTFLAGS");
    //> [RS-W1015]: "Using `env::set_var` with a string literal"
    env::set_var("RUSTFLAGS", "1");
    //> [RS-W1015]: "Using `env::remove_var` with a string literal"
    env::remove_var("RUSTFLAGS");
}

#[rustfmt::skip]
fn no_match() {
    static RUSTFLAGS: &str = "RUSTFLAGS";
    let _ = std::env::var(RUSTFLAGS);
    std::env::set_var(RUSTFLAGS, "-1");
}
