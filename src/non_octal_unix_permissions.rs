use std::{
    fs::{DirBuilder, File, OpenOptions, Permissions},
    os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt},
};

#[rustfmt::skip]
fn test() {
    // OpenOptionsExt::mode
    let mut options = OpenOptions::new();
        options.mode(440);
    options.mode(0o400);

    // PermissionsExt::from_mode
        let _permissions = Permissions::from_mode(647);
    let _permissions = Permissions::from_mode(0o000);

    // PermissionsExt::set_mode
    let f = File::create("foo.txt").unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();

        permissions.set_mode(644);
    permissions.set_mode(0o704);

    // DirBuilderExt::mode
    let mut builder = DirBuilder::new();
        builder.mode(755);
    builder.mode(0o406);
}
