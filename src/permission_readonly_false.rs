use std::fs::File;

fn main() {
    let f = File::create("foo.txt").unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    //> [RS-S1016]: "Call that sets global write permission on file"
    permissions.set_readonly(false);
}
