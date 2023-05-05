//> scatr-check: RS-W1011

#[rustfmt::skip]
mod must_use_unit {
    type T = ();

   /**///> [RS-W1011]: "Function that is annotated with `#[must_use]` returns `()`"
   #[must_use] fn a() {}

   /**///> [RS-W1011]: "Function that is annotated with `#[must_use]` returns `()`"
   #[must_use] pub fn b() {}

   /**///> [RS-W1011]: "Function that is annotated with `#[must_use]` returns `()`"
   #[must_use] pub fn c() -> T {}

   /**///> [RS-W1011]: "Function that is annotated with `#[must_use]` returns `()`"
   #[must_use] #[cfg(feature = "publish")] fn d() -> () {}

   /**///> [RS-W1011]: "Function that is annotated with `#[must_use]` returns `()`"
   #[must_use = "With note"] pub fn must_use_with_note() {}

    #[must_use] pub async fn yield_now() {}
}
