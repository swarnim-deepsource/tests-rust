#[rustfmt::skip]
mod tests {

    fn trivial() {
        let x = Some(2);
               x.iter().fold(None, |acc, y| acc.or(Some(y)));

        let y = Some(0);
               x.into_iter().fold(0, |acc: i32, y| acc.wrapping_add(y));
    }

    mod no_match {
        struct Some<T>(T);
        impl<T> Some<T> {
            fn iter(&self) -> std::option::Iter<'_, T> {
                unimplemented!()
            }
        }

        fn no_match() {
            let x = Some(2);
            x.iter().fold(0, |_, _| 0);
        }
    }
}
