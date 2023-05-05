/*
disables = [ "large-enum-variant" ]
*/

enum LargeEnumVariant {
    Smol(String),
    Smol2 { a: i32, b: u32 },
    Mid([usize; 10]),
    Large([usize; 10000]),
}

enum LargeEnumVariant {
    Smol(String),
    Mid([usize; 10]),
    Large([&'static str; 10000]),
}

enum LargeEnumVariant {
    Smol(String),
    Mid([usize; 10]),
    Large([i32; 10000]),
}

enum LargeEnumVariant {
    Smol { a: i32, b: u32 },
    Smol1(&'static str),
    Smol2(i32),
    Smol3(&[usize; 10]),
    Smol4((i32, u32)),
    Smol5 { a: i32, b: u32 },
    Smol6 { a: i32, b: u32, c: i32, d: String },
    Mid([usize; 10]),
    Large([i32; 1000]),
    Large1([i32; 2000]),
}

enum LargeEnumVariant {
    Smol(String),
    Mid([usize; 10]),
    Large([i32; 1000]),
    Large1([i32; 2000]),
    Large2([i32; 3000]),
}
