#[rustfmt::skip]
mod test {
    fn trivial() {
        use std::{iter::once, convert::identity};
        //> [RS-W1089]: "The call to `.map()` will have no effect on the result of `.count()`"
        once(1).map(identity).count();
    }
}
