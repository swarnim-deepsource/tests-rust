// This file tests lints:
// - forget_drop_copy
// - forget_drop_ref
// - forget_drop_no_trait
//
// Ensure that all lints are exclusive of each other and
// are not raised on the same spans.
#[rustfmt::skip]
mod drop_forget_copy_ref {

    #[derive(Clone, Copy)]
    struct S {}
    #[derive(Default)]
    struct T {}

    fn trivial() {
        let s1 = S {};
        let s2 = s1;
        let s3 = &s1;
        let mut s4 = s1;
        let ref s5 = s1;
        let t1 = T {};
              std::mem::forget(s1);
              std::mem::drop(s1);

               std::mem::forget(s2);
               std::mem::drop(s2);

               std::mem::forget(s3);
                std::mem::drop(s3);
                std::mem::forget(s4);
                std::mem::drop(s4);

                std::mem::forget(s5);
                std::mem::drop(s5);
                std::mem::drop(t1);

    }

    fn non_standard_path() {
        use std::mem as memstuff;
        use std::mem::{forget as whoopsie, drop as doopsie};

                whoopsie(99);
                doopsie(99);

                memstuff::forget(99);
                memstuff::drop(99);
    }
}
