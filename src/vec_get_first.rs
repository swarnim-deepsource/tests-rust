use std::collections::{HashMap, VecDeque};

fn main() {
    let mut v = vec![1, 2, 3, 4];
    //> [RS-W1116]: "Found `get(0)`, use `first()` instead"
    let one = v.get(0);
    //> [RS-W1116]: "Found `get_mut(0)`, use `first_mut()` instead"
    let one_mut = v.get_mut(0);
    let mut dq = VecDeque::from([1, 2, 3, 4]);
    //> [RS-W1116]: "Found `get(0)`, use `front()` instead"
    let one = dq.get(0);
    //> [RS-W1116]: "Found `get_mut(0)`, use `front_mut()` instead"
    let one_mut = dq.get_mut(0);
    let h: HashMap<usize, usize> = HashMap::new();
    let one = h.get(&0);
}
