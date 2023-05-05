#[rustfmt::skip]
mod test {

    fn trivial() {
          let _ = "12345".replace('1', "1");
          let _ = "12345".replace("12", "12");
          let _ = String::default().replace("12", "12");

          let _ = "12345".replacen('1', "1", 1);
          let _ = "12345".replacen("12", "12", 1);
          let _ = String::default().replacen("12", "12", 1);

          let _ = "12345".replacen('\t', "\t", 1);

        let x = "abc";
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
