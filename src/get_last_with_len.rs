#[rustfmt::skip]
mod test {
    fn dont_use_last() {
        let x = vec![2, 3, 5];
                let _ = x.get(x.len() - 1);
    }

    fn dont_use_last_deque() {
        use std::collections::VecDeque;
        let x = VecDeque::from([2, 3, 5]);
                let _ = x.get(x.len() - 1);
    }

    fn indexing_two_from_end() {
        let x = vec![2, 3, 5];
        let _ = x.get(x.len() - 2);
    }

    fn index_into_last() {
        let x = vec![2, 3, 5];
        let _ = x[x.len() - 1];
    }

    fn use_last_with_different_vec_length() {
        let x = vec![2, 3, 5];
        let y = vec!['a', 'b', 'c'];
        let _ = x.get(y.len() - 1);
    }

}
