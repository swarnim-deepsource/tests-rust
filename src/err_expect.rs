struct MyTypeNonDebug;

#[derive(Debug)]
struct MyTypeDebug;

#[rustfmt::skip]
fn main() {
    let test_debug: Result<MyTypeDebug, u32> = Ok(MyTypeDebug);
    //> [RS-W1071]: "Called `.err().expect()` on a `Result` value, call `.expect_err()` directly instead"
    test_debug.err().expect("Testing debug type");

    let test_debug2: Result<MyTypeDebug, u32> = Ok(MyTypeDebug);
    //> [RS-W1071]: "Called `.err().unwrap()` on a `Result` value, call `.unwrap_err()` directly instead"
    test_debug2.err().unwrap();

    unsafe {
        let test_debug3: Result<MyTypeDebug, u32> = Ok(MyTypeDebug);
       //> [RS-W1071]: "Called `.err().unwrap_unchecked()` on a `Result` value, call `.unwrap_err_unchecked()` directly instead"
        test_debug3.err().unwrap_unchecked();
    }

    let test_debug4: Result<MyTypeDebug, u32> = Ok(MyTypeDebug);
   //> [RS-W1071]: "Called `.err().map()` on a `Result` value, call `.map_err()` directly instead"
    test_debug4.err().map(|t| t * 2);

    let test_debug5: Result<MyTypeDebug, u32> = Err(2);
    // "WON'T RAISE ATM" [RS-W1071]: "Called `.err().contains()` on a `Result` value, call `.contains_err()` directly instead"
    // let _ = test_debug5.err().contains(&2);

    let test_non_debug: Result<MyTypeNonDebug, u32> = Ok(MyTypeNonDebug);
    test_non_debug.err().expect("Testing non debug type");
}
