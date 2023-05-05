#[rustfmt::skip]
mod test {
    // Checking `if` cases
    fn not_good(x: &str) -> i32 {
                if let Some(y) = x.parse().ok() { y } else { 0 }
    }

    fn good(x: &str) -> i32 {
        if let Ok(y) = x.parse() { y } else { 0 }
    }

    #[rustfmt::skip]
    fn absurd(x: &str) -> i32 {
                if let Some(y) = x   .   parse()   .   ok   ()    {
            return y;
        };
        0
    }

    // Checking `while` cases
    struct Wat {
        counter: i32,
    }

    impl Wat {
        fn next(&mut self) -> Result<i32, &str> {
            self.counter += 1;
            if self.counter < 5 {
                Ok(self.counter)
            } else {
                Err("Oh no")
            }
        }
    }

    fn not_good_while(x: i32) {
        let mut wat = Wat { counter: x };
                while let Some(a) = wat.next().ok() {
            println!("{}", a);
        }
    }

    fn good_while(x: i32) {
        let mut wat = Wat { counter: x };
        while let Ok(a) = wat.next() {
            println!("{}", a);
        }
    }
}
