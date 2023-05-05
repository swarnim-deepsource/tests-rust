#[rustfmt::skip]
mod tests {
    fn strs() {
        let st = "hello";
                if st == "" { }
                if "" == st { }
    }

    fn vecs() {
        // let v: Vec<u32> = vec![];
        // let s: &[u32] = &v[0..3];

        // [RS-W1102]
        // if s == [] { }
        // [RS-W1102]
        // if v == [] { }
    }

    fn negation_msg() {
        let st = "hello";

                if st != "" { }
                if "" != st { }
    }
}
