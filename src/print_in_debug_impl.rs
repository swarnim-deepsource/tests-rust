use std::fmt::{self, Debug};

struct A;

impl Debug for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //> [RS-W1133]: "Using `print!` in implementation of `Debug`"
        print!("hello");
        Ok(())
    }
}

struct B;

impl Debug for B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //> [RS-W1133]: "Using `eprintln!` in implementation of `Debug`"
        eprintln!("hello");
        Ok(())
    }
}

struct C;

impl Debug for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //> [RS-W1133]: "Using `std::println!` in implementation of `Debug`"
        std::println!("hello");
        Ok(())
    }
}

struct D;

impl Debug for D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //> [RS-W1133]: "Using `std::eprint!` in implementation of `Debug`"
        std::eprint!("hello");
        Ok(())
    }
}
