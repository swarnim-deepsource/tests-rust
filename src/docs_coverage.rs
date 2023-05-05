//> scatr-check: RS-D1001

/**///> [RS-D1001]: "Undocumented public struct"
pub struct A {
    b: i32,
    c: u32,
}

/** Hello */ //> [RS-D1001]: "Undocumented public field in struct"
pub struct B {
    pub b: i32,
    c: u32,
}

/** Hello again */ //> [RS-D1001]: "Undocumented public fields in struct"
pub struct C {
    pub b: i32,
    pub c: u32,
}

/**///> [RS-D1001]: "Undocumented public enumeration"
pub enum D {
    TupleE(i32),
    UnitE,
    F(u32),
}

/** Hello a third time */ //> [RS-D1001]: "Undocumented public variant in enumeration"
pub enum E {
    RecordE { a: i32, b: u32 },
    TupleE(i32),
}

/** If I say hello again you'll kill me */ //> [RS-D1001]: "Undocumented public variants in enumeration"
pub enum F {
    RecordE { a: i32, b: u32 },
    RecordF { a: i32, b: u32 },
    UnitE,
}

/**///> [RS-D1001]: "Undocumented public union"
pub union G {
    b: i32,
    c: u32,
}

/** I'm gonna say it anyways */ //> [RS-D1001]: "Undocumented public field in union"
pub union H {
    pub b: i32,
    c: u32,
}

/** Hello, is there anybody in there */ //> [RS-D1001]: "Undocumented public fields in union"
pub union I {
    pub b: i32,
    pub c: u32,
}

/**///> [RS-D1001]: "Undocumented public trait"
pub trait TestDocs {
    const TEST: usize;
    type Test;
    /**///> [RS-D1001]: "Undocumented public trait method"
    fn test();
}

/**///> [RS-D1001]: "Undocumented public static item"
pub static S: &str = "hello world";

/**///> [RS-D1001]: "Undocumented public constant"
pub const ANSWER: usize = 42;

/**///> [RS-D1001]: "Undocumented public type alias"
pub type Test = (usize, usize);

/**///> [RS-D1001]: "Undocumented public module"
pub mod foo;

/**///> [RS-D1001]: "Undocumented public function"
pub fn test() {}

/// This is a doc comment
pub struct J {
    b: i32,
    /** This is also a doc comment */
    pub c: u32,
}

struct PrivateA {}

#[doc = "This is a doc comment replacement"]
pub enum K {
    /// Another comment
    RecordE { a: i32, b: u32 },
    #[doc(hidden)]
    E(i32),
    #[doc = "This is a doc comment replacement"]
    F(u32),
}

enum PrivateD {
    E,
    F(u32),
}

#[doc(hidden)]
pub union L {
    h: i32,
    #[doc(hidden)]
    pub i: u32,
}

union PrivateE {
    x: i32,
    y: u32
}

#[doc(hidden)]
pub trait TestDocs2 {
    const TEST: usize;
    type Test;
    fn test();
}

#[doc(hidden)]
pub static S1: &str = "hello world";

#[doc(hidden)]
pub const ANSWER1: usize = 42;

#[doc(hidden)]
pub type Test1 = (usize, usize);

mod t {
    #[doc(hidden)]
    pub mod foo;
}

#[doc(hidden)]
pub fn test1() {}
