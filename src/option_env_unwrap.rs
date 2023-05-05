#[rustfmt::skip]
fn trivial() {
        let _ = option_env!("X").unwrap();
}

#[rustfmt::skip]
fn qualified() {
        let _ = std::option_env!("X").unwrap();
        let _ = core::option_env!("X").unwrap();
        let _ = ::std::option_env!("X").unwrap();
        let _ = ::core::option_env!("X").unwrap();
}

fn no_match() {
    let _ = option_env!("X").unwrap_or("abc");
}
