/* msrv = "1.48.0"
*/

#[rustfmt::skip]
mod test {
    use std::collections::VecDeque;

    fn repro(dq: &mut VecDeque<i32>, sz: usize) {
        for i in 0..sz {
            dq.push_back(i as _);
        }
        dq.make_contiguous();
        for _ in 0..sz {
            dq.pop_front();
        }
    }

    fn repro_call_expr(dq: &mut VecDeque<i32>, sz: usize) {
        for i in 0..sz {
            dq.push_back(i as _);
        }
        VecDeque::make_contiguous(dq);
        for _ in 0..sz {
            dq.pop_front();
        }
    }
}
