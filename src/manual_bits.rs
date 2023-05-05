use std::mem;

fn foo() {
    //> [RS-W1132]
    let s1 = mem::size_of::<usize>() * 8;
    //> [RS-W1132]
    let s2 = mem::size_of::<u32>() * 8;
    //> [RS-W1132]
    let s3 = mem::size_of::<i128>() * 8;
    //> [RS-W1132]
    let s4 = mem::size_of::<i64>() * 8;
}
