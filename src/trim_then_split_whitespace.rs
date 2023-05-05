#[rustfmt::skip]
mod tests {
    fn trivial() {
        let s = " a b c";
        //> [RS-P1005]: "Calling `.trim()` before `.split_whitespace()` is redundant"
        s.trim().split_whitespace().collect::<Vec<_>>();

        //> [RS-P1005]: "Calling `.trim_start()` before `.split_whitespace()` is redundant"
        s.trim_start().split_whitespace().collect::<Vec<_>>();

        //> [RS-P1005]: "Calling `.trim_end()` before `.split_whitespace()` is redundant"
        s.trim_end().split_whitespace().collect::<Vec<_>>();
    }

    fn no_match() {
        let s = "a b c ";
        s.trim().split_ascii_whitespace().collect::<Vec<_>>();
        s.trim().split_terminator(' ').collect::<Vec<_>>();
    }

    fn types() {
        let s = String::from(" a b c ");
        //> [RS-P1005]: "Calling `.trim()` before `.split_whitespace()` is redundant"
        s.trim().split_whitespace().collect::<Vec<_>>();
    }
}
