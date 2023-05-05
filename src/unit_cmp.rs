#[rustfmt::skip]
mod test {
    fn trivial() {
       //> [RS-E1016]: "Comparison between unit-types detected, this will always be `true`"
        if { true; } == { false; } {}

       //> [RS-E1016]: "Comparison between unit-types detected, this will always be `true`"
        if { true; } <= { false; } {}

       //> [RS-E1016]: "Comparison between unit-types detected, this will always be `true`"
        if { true; } >= { false; } {}

       //> [RS-E1016]: "Comparison between unit-types detected, this will always be `false`"
        if { true; } > { false; } {}

       //> [RS-E1016]: "Comparison between unit-types detected, this will always be `false`"
        if { true; } != { false; } {}
    }

    fn no_match() {
        let a = 2;
        let b = 3;
        if a == b {}
    }
}
