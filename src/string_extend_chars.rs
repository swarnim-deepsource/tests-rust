#[rustfmt::skip]
mod test {

    fn trivial() {
        let mut target = "hello".to_owned();
        let mut s1 = "world";
        let mut s2 = "world".to_owned();

        //> [RS-W1095]: "Calling `_.extend(s.chars())`, use `_.push_str(s)` instead"
        target.extend(s1.chars());
        //> [RS-W1095]: "Calling `_.extend(s.chars())`, use `_.push_str(&s)` instead"
        target.extend(s2.chars());
    }

    fn no_match() {
        let mut target = vec!['a', 'b', 'c'];
        let mut s = "world";

        target.extend(s.chars());
    }
}
