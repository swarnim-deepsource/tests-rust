#[rustfmt::skip]
mod test {
    fn trivial() {
               if { true; } == { false; } {}

               if { true; } <= { false; } {}

               if { true; } >= { false; } {}

               if { true; } > { false; } {}

               if { true; } != { false; } {}
    }

    fn no_match() {
        let a = 2;
        let b = 3;
        if a == b {}
    }
}
