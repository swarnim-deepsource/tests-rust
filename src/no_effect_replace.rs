#[rustfmt::skip]
mod test {

    fn trivial() {
     //> [RS-W1094]: "This call to `replace` has no effect, it replaces the text with itself"
     let _ = "12345".replace('1', "1");
     //> [RS-W1094]: "This call to `replace` has no effect, it replaces the text with itself"
     let _ = "12345".replace("12", "12");
     //> [RS-W1094]: "This call to `replace` has no effect, it replaces the text with itself"
     let _ = String::default().replace("12", "12");

     //> [RS-W1094]: "This call to `replacen` has no effect, it replaces the text with itself"
     let _ = "12345".replacen('1', "1", 1);
     //> [RS-W1094]: "This call to `replacen` has no effect, it replaces the text with itself"
     let _ = "12345".replacen("12", "12", 1);
     //> [RS-W1094]: "This call to `replacen` has no effect, it replaces the text with itself"
     let _ = String::default().replacen("12", "12", 1);

     //> [RS-W1094]: "This call to `replacen` has no effect, it replaces the text with itself"
     let _ = "12345".replacen('\t', "\t", 1);

        let x = "abc";
     //> [RS-W1094]: "This call to `replacen` has no effect, it replaces the text with itself"
     let _ = "abcabc".replacen(x, x, 2);
    }

    struct Replace {}

    impl Replace {
        fn replace(&self, a: &str, b: &str) {
            unimplemented!()
        }
    }

    fn no_match() {
        let _ = "12345".replace("12", "22");
        let _ = "12345".replacen("12", "22", 1);
        let _ = "12345".replacen('1', "11", 1);

        Replace {}.replace("1", "1");

        let mut x = ["abc", "def"].iter();
        let _ = "abcabc".replacen(x.next().unwrap(), x.next().unwrap(), 2);
    }

}
