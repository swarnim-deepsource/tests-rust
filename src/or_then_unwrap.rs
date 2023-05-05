/* disables = ["map-identity"] */ //> scatr-check: RS-W1092
#[rustfmt::skip]
mod test {
    struct SomeStruct;
    impl SomeStruct {
        fn or(self, _: Option<Self>) -> Self {
            self
        }
        fn unwrap(&self) {}
    }

    struct SomeOtherStruct;
    impl SomeOtherStruct {
        fn or(self) -> Self {
            self
        }
        fn unwrap(&self) {}
    }

    fn trivial() {
        let option: Option<&str> = None;
        //> [RS-W1092]: "Calling `.or(Some(..)).unwrap()`, try `.unwrap_or(..)` instead"
        let _ = option.or(Some("fallback")).unwrap();

        let result: Result<&str, &str> = Err("Error");
        //> [RS-W1092]: "Calling `.or(Ok(..)).unwrap()`, try `.unwrap_or(..)` instead"
        let _ = result.or::<&str>(Ok("fallback")).unwrap();

        let option: Option<&str> = None;
        //> [RS-W1092]: "Calling `.or(Some(..)).unwrap()`, try `.unwrap_or(..)` instead"
        let _ = option.map(|v| v).or(Some("fallback")).unwrap().to_string().chars();

    }

    fn no_match() {
        // Not Option/Result
        let instance = SomeStruct {};
        let _ = instance.or(Some(SomeStruct {})).unwrap();

        // or takes no argument
        let instance = SomeOtherStruct {};
        let _ = instance.or().unwrap();

        // None in or
        let option: Option<&str> = None;
        let _ = option.or(None).unwrap();

        // Not Err in or
        let result: Result<&str, &str> = Err("Error");
        let _ = result.or::<&str>(Err("Other Error")).unwrap();

        // other function between
        let option: Option<&str> = None;
        //> [RS-W1018]: "Unnecessary map of the identity function"
        let _ = option.or(Some("fallback")).map(|v| v).unwrap();
    }
}
