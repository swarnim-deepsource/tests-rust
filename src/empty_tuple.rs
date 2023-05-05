#![rustfmt::skip]
//> [RS-W1125]: "Found empty tuple struct"
struct A();

enum B {
    //> [RS-W1125]: "Found empty tuple enum variant"
    C()
}
