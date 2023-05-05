#[rustfmt::skip]
fn f() {
    let s = "str";
   //> [RS-P1002]: "Called `.bytes().nth(..)` when `.as_bytes().get(..)` would suffice"
    s.bytes().nth(2);

   //> [RS-P1002]: "Called `.bytes().nth(..)` when `.as_bytes().get(..)` would suffice"
    "str".bytes().nth(2);

    let i = FalsePositive { x: 0u32 };
    i.bytes().nth(2);
}

struct FalsePositive {
    x: u32,
}

impl FalsePositive {
    fn bytes(&self) -> &Self {
        self
    }
    fn nth(&self, _i: u32) -> Option<u8> {
        Some(_i as u8)
    }
}
