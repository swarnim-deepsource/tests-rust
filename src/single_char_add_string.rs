#[rustfmt::skip]
mod tests {
    fn f() {
        let mut x = String::from("hello");

        // ok, don't raise
        x.push_str("world");
        x.push_str(r#"
        "#);

        //> [RS-P1004]: "Calling `push_str` with single-character string literal `\"!\"`, use `push` with a character literal instead"
        x.push_str("!");

        //> [RS-P1004]: "Calling `insert_str` with single-character string literal `\"!\"`, use `insert` with a character literal instead"
        x.insert_str(10, "!");
    }
}
