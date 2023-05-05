#[rustfmt::skip]
fn f() {
    use std::fs;
        ::std::fs::create_dir("a/b");
        std::fs::create_dir("a/b");
        fs::create_dir("a/b");
}

#[rustfmt::skip]
mod qualified {
    fn f() {
        use std::fs::create_dir;
                create_dir("a/b");
    }
    fn g() {
        use std::fs::create_dir as my_create_dir;
                my_create_dir("a/b");
    }
}

mod custom_no_match {
    fn create_dir(s: &str) {}

    fn no_match() {
        create_dir("a/b");
    }
}
