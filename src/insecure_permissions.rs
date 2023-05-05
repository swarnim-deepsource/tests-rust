use std::{
    fs::{DirBuilder, File, OpenOptions, Permissions},
    os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt},
};

#[rustfmt::skip]
fn test() {
    // OpenOptionsExt::mode
    let mut options = OpenOptions::new();
    options.mode(0o211);
    //> [RS-A1001]: "Using `OpenOptions::mode` with insecure permissions: `0o777`"
    options.mode(0o777);

    let _permissions = Permissions::from_mode(0o000);
    //> [RS-A1001]: "Using `Permissions::from_mode` with insecure permissions: `0o777`"
    let _permissions = Permissions::from_mode(0o777);
    let _permissions = Permissions::from_mode(0o755);

    // PermissionsExt::set_mode
    let f = File::create("foo.txt").unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();

    //> [RS-A1001]: "Using `Permissions::set_mode` with insecure permissions: `0o777`"
    permissions.set_mode(0o777);
    //> [RS-A1001]: "Using `Permissions::set_mode` with insecure permissions: `0o766`"
    permissions.set_mode(0o766);

    // DirBuilderExt::mode
    let mut builder = DirBuilder::new();
    builder.mode(0o406);
    //> [RS-A1001]: "Using `DirBuilder::mode` with insecure permissions: `0o777`"
    builder.mode(0o777);
}
