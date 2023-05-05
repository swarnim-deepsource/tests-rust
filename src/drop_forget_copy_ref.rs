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
      //> [RS-E1011]: "Using `std::mem::forget` with type that implements `Copy`"
        std::mem::forget(s1);
      //> [RS-E1011]: "Using `std::mem::drop` with type that implements `Copy`"
        std::mem::drop(s1);

       //> [RS-E1011]: "Using `std::mem::forget` with type that implements `Copy`"
        std::mem::forget(s2);
       //> [RS-E1011]: "Using `std::mem::drop` with type that implements `Copy`"
        std::mem::drop(s2);

       //> [RS-E1010]: "Using `std::mem::forget` with a reference"
        std::mem::forget(s3);
        //> [RS-E1010]
        std::mem::drop(s3);
        //> [RS-E1011]
        std::mem::forget(s4);
        //> [RS-E1011]
        std::mem::drop(s4);

        //> [RS-E1010]
        std::mem::forget(s5);
        //> [RS-E1010]
        std::mem::drop(s5);
        //> [RS-E1021]
        std::mem::drop(t1);

    }

    fn non_standard_path() {
        use std::mem as memstuff;
        use std::mem::{forget as whoopsie, drop as doopsie};

        //> [RS-E1011]
        whoopsie(99);
        //> [RS-E1011]
        doopsie(99);

        //> [RS-E1011]
        memstuff::forget(99);
        //> [RS-E1011]
        memstuff::drop(99);
    }
}
