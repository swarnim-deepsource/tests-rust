use std::fmt::{self, Display};

struct A;

impl Display for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //> [RS-E1034]: "Using `print!` in implementation of `Display`"
        print!("hello");
        Ok(())
    }
}

struct B;

impl Display for B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //> [RS-E1034]: "Using `eprintln!` in implementation of `Display`"
        eprintln!("hello");
        Ok(())
    }
}

struct C;

impl Display for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //> [RS-E1034]: "Using `std::println!` in implementation of `Display`"
        std::println!("hello");
        Ok(())
    }
}
struct D;

impl Display for D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //> [RS-E1034]: "Using `std::eprint!` in implementation of `Display`"
        std::eprint!("hello");
        Ok(())
    }
}
