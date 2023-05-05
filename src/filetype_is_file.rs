#[rustfmt::skip]
fn f() -> std::io::Result<()> {
    use std::{fs, ops::BitOr};

    // !filetype.is_dir()
    //> [RS-A1000]: "Using `FileType::is_file` instead of `!FileType::is_dir`"
    if fs::metadata("foo.txt")?.file_type().is_file() {
    }

    // false positive of filetype.is_dir()
    // in this case the user probably wants to do something 
    // with the output of is_file but we don't have a way of knowing that
    //> [RS-A1000]: "Using `FileType::is_file` instead of `!FileType::is_dir`"
    if fs::metadata("foo.txt")?.file_type().is_file().bitor(true) {
    }

    // negated message for !filetype.is_file()
    //
    // if methods are chained to `is_file`, they can potentially
    // change the output (for example, `is_file().then(|| Some(false)).uwnrap()`
    //
    // we lint only on the basic case:
    //> [RS-A1000]: "Using `!FileType::is_file` instead of `FileType::is_dir`"
    if !fs::metadata("foo.txt")?.file_type().is_file() {
    }

    Ok(())
}
