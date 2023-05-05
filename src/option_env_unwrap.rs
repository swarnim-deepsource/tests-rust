#[rustfmt::skip]
fn trivial() {
    //> [RS-W1016]: "Called `unwrap` on `option_env!` macro"
    let _ = option_env!("X").unwrap();
}

#[rustfmt::skip]
fn qualified() {
    //> [RS-W1016]: "Called `unwrap` on `option_env!` macro"
    let _ = std::option_env!("X").unwrap();
    //> [RS-W1016]: "Called `unwrap` on `option_env!` macro"
    let _ = core::option_env!("X").unwrap();
    //> [RS-W1016]: "Called `unwrap` on `option_env!` macro"
    let _ = ::std::option_env!("X").unwrap();
    //> [RS-W1016]: "Called `unwrap` on `option_env!` macro"
    let _ = ::core::option_env!("X").unwrap();
}

fn no_match() {
    let _ = option_env!("X").unwrap_or("abc");
}
