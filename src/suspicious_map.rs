#[rustfmt::skip]
mod test {
    fn trivial() {
        use std::{iter::once, convert::identity};
                once(1).map(identity).count();
    }
}
