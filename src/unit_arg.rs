// inherits clippys test suite shamelessly
#[rustfmt::skip]
mod test {
    use std::fmt::Debug;

    fn foo<T: Debug>(t: T) {
        println!("{:?}", t);
    }

    fn foo3<T1: Debug, T2: Debug, T3: Debug>(t1: T1, t2: T2, t3: T3) {
        println!("{:?}, {:?}, {:?}", t1, t2, t3);
    }

    struct Bar;

    impl Bar {
        fn bar<T: Debug>(&self, t: T) {
            println!("{:?}", t);
        }
    }

    fn baz<T: Debug>(t: T) {
        foo(t);
    }

    trait Tr {
        type Args;
        fn do_it(args: Self::Args);
    }

    struct A;
    impl Tr for A {
        type Args = ();
        fn do_it(_: Self::Args) {}
    }

    struct B;
    impl Tr for B {
        type Args = <A as Tr>::Args;

        fn do_it(args: Self::Args) {
            //> [RS-E1015]
            A::do_it(args)
        }
    }

    fn bad() {
        //> [RS-E1015]
        foo({ 1; });
        //> [RS-E1015]
        foo(foo(1));
        //> [RS-E1015]
        foo({ foo(1); foo(2); });
        let b = Bar;
        //> [RS-E1015]
        b.bar({ 1; }); 
        //> [RS-E1015]
        taking_multiple_units(foo(0), foo(1));
        //> [RS-E1015]
        None.or(Some(foo(2)));
        //> [RS-E1015]
        foo(foo(()));
    }

    fn ok() {
        foo(());
        foo(1);
        foo({ 1 });
        foo3("a", 3, vec![3]);
        let b = Bar;
        b.bar({ 1 });
        b.bar(());
        question_mark();
        // once we have DFA, we can ensure this is not raised
        // let named_unit_arg = ();
        // foo(named_unit_arg);   
        baz(());
        B::do_it(());
    }

    fn question_mark() -> Result<(), ()> {
        Ok(Ok(())?)?;
        Ok(Ok(()))??;
        Ok(())
    }

    #[allow(dead_code)]
    mod issue_2945 {
        fn unit_fn() -> Result<(), i32> {
            Ok(())
        }

        fn fallible() -> Result<(), i32> {
            Ok(unit_fn()?)
        }
    }

    #[allow(dead_code)]
    fn returning_expr() -> Option<()> {
        //> [RS-E1015]
        Some(foo(1))
    }

    fn taking_multiple_units(a: (), b: ()) {}

    fn main() {
        bad();
        ok();
    }
}
