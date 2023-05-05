/*
msrv = "1.42.0"
disables = [ "closure-redundant-tail-expr" ]
*/

#[rustfmt::skip]
mod tests {
    fn option_methods() {
        let opt = Some(0);
                let _ = opt.map(|x| { x + 1 }).unwrap_or_else(|| std::process::exit(0));
                let _ = opt.map(|x| { x + 1 }).unwrap_or(0);
    }
    fn result_methods() {
        let res: Result<i32, ()> = Ok(1);

                let _ = res.map(|x| { x + 1 }).unwrap_or_else(|_e| std::process::exit(0));
                let _ = res.map(|x| { x + 1 }).unwrap_or(0);
    }

    enum MyResult<T, E> { Ok(T), Err(E)}
    impl<T, E> MyResult<T, E> {
        fn map<U, F: FnOnce(T) -> U>(self, _: F) -> MyResult<U, E> {
            unimplemented!()
        }
        fn unwrap_or(self, _: T) -> T {
            unimplemented!()
        }
        fn unwrap_or_else<F: FnOnce(E) -> T>(self, _: F) -> T {
            unimplemented!()
        }
    }

    fn no_match() {
        let my_res1: MyResult<u32, ()> = MyResult::Ok(1u32);
                let _ = my_res1.map(|x| { x + 1 }).unwrap_or_else(|_e| std::process::exit(0));

        let my_res2: MyResult<u32, ()> = MyResult::Ok(1u32);
                let _ = my_res2.map(|x| { x + 1 }).unwrap_or(0);
 
    }
}
