/* disables = [ "nth-zero" ]
 */
#[rustfmt::skip]
mod test {
    fn iter_nth() {
        use std::collections::VecDeque;

        let mut some_vec = vec![0, 1, 2, 3];
        let mut some_array: [u8; 4] = [0, 1, 2, 3];
        let mut boxed_slice: Box<[u8]> = Box::new([0, 1, 2, 3]);
        let mut some_vec_deque: VecDeque<_> = some_vec.iter().cloned().collect();

        {
            // Make sure we lint `.iter()` for relevant types.
                        let bad_vec = some_vec.iter().nth(3);
                        let bad_array = some_array.iter().nth(3);
                        let bad_slice = &some_vec[..].iter().nth(3);
                        let bad_boxed_slice = boxed_slice.iter().nth(3);
                        let bad_vec_deque = some_vec_deque.iter().nth(3);
        }

        {
                        let bad_vec = some_vec.iter_mut().nth(3);
        }

        {
                        let bad_array = some_array.iter_mut().nth(3);
        }


        {
                        let bad_slice = &some_vec[..].iter_mut().nth(3);
        }

        {
                        let bad_vec_deque = some_vec_deque.iter_mut().nth(3);
        }

        {
                        let _ = some_vec.iter_mut().nth(0);
        }
    }
}
