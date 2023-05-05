#![rustfmt::skip]
struct Foo {
    buffer: [u8],
}

impl Foo {
    fn size(&self) -> usize {
        //> [RS-W1123]: "Calculating size of reference to `&self`"
        std::mem::size_of_val(&self)
    }

    fn correct_size(&self) -> usize {
        std::mem::size_of_val(self)
    }
}
