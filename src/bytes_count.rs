#[rustfmt::skip]
fn f() {
    let s = "str";
   //> [RS-P1001]: "Called `.bytes().count()` when `.len()` would suffice"
    s.bytes().count();

   //> [RS-P1001]: "Called `.bytes().count()` when `.len()` would suffice"
    "string".bytes().count();

    let i = FalsePositive { x: 0u32 };
    i.bytes().count();
}

struct FalsePositive {
    x: u32,
}

impl FalsePositive {
    fn bytes(&self) -> &Self {
        self
    }
    fn count(&self) -> u32 {
        self.x
    }
}