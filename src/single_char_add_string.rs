#[rustfmt::skip]
mod tests {
    fn f() {
        let mut x = String::from("hello");

        // ok, don't raise
        x.push_str("world");
        x.push_str(r#"
        "#);

                x.push_str("!");

                x.insert_str(10, "!");
    }
}
