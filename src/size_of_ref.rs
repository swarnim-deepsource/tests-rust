#![rustfmt::skip]
struct Foo {
    buffer: [u8],
}

impl Foo {
    fn size(&self) -> usize {
                std::mem::size_of_val(&self)
    }

    fn correct_size(&self) -> usize {
        std::mem::size_of_val(self)
    }
}
