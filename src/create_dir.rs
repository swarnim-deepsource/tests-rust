#[rustfmt::skip]
fn f() {
    use std::fs;
    //> [RS-W1032]: "Using `::std::fs::create_dir` instead of `std::fs::create_dir_all`"
    ::std::fs::create_dir("a/b");
    //> [RS-W1032]: "Using `std::fs::create_dir` instead of `std::fs::create_dir_all`"
    std::fs::create_dir("a/b");
    //> [RS-W1032]: "Using `fs::create_dir` instead of `std::fs::create_dir_all`"
    fs::create_dir("a/b");
}

#[rustfmt::skip]
mod qualified {
    fn f() {
        use std::fs::create_dir;
        //> [RS-W1032]: "Using `create_dir` instead of `std::fs::create_dir_all`"
        create_dir("a/b");
    }
    fn g() {
        use std::fs::create_dir as my_create_dir;
        //> [RS-W1032]: "Using `my_create_dir` instead of `std::fs::create_dir_all`"
        my_create_dir("a/b");
    }
}

mod custom_no_match {
    fn create_dir(s: &str) {}

    fn no_match() {
        create_dir("a/b");
    }
}
