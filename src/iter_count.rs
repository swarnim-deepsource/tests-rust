#[rustfmt::skip]
mod test {
    use std::collections::{VecDeque, HashSet, HashMap, BTreeSet, BTreeMap, LinkedList, BinaryHeap};
    fn trivial() {
        let mut vec = vec![0, 1, 2, 3];
        let mut boxed_slice: Box<[u8]> = Box::new([0, 1, 2, 3]);
        let mut array: [u8; 4] = [0, 1, 2, 3];
        let mut vec_deque: VecDeque<_> = vec.iter().cloned().collect();
        let mut hash_set = HashSet::new();
        let mut hash_map = HashMap::new();
        let mut b_tree_map = BTreeMap::new();
        let mut b_tree_set = BTreeSet::new();
        let mut linked_list = LinkedList::new();
        let mut binary_heap = BinaryHeap::new();
        hash_set.insert(1);
        hash_map.insert(1, 2);
        b_tree_map.insert(1, 2);
        b_tree_set.insert(1);
        linked_list.push_back(1);
        binary_heap.push(1);
                &vec[..].iter().count();
                vec.iter().count();
                boxed_slice.iter().count();
                array.iter().count();
                vec_deque.iter().count();
                hash_set.iter().count();
                hash_map.iter().count();
                b_tree_map.iter().count();
                b_tree_set.iter().count();
                linked_list.iter().count();
                binary_heap.iter().count();

                vec.iter_mut().count();
                &vec[..].iter_mut().count();
                array.iter_mut().count();
                vec_deque.iter_mut().count();
                hash_map.iter_mut().count();
                b_tree_map.iter_mut().count();
                linked_list.iter_mut().count();

                &vec[..].into_iter().count();
                vec.into_iter().count();
                array.into_iter().count();
                vec_deque.into_iter().count();
                hash_set.into_iter().count();
                hash_map.into_iter().count();
                b_tree_map.into_iter().count();
                b_tree_set.into_iter().count();
                linked_list.into_iter().count();
                binary_heap.into_iter().count();
    }

    /// Struct to generate false positives for things with `.iter()`.
    #[derive(Copy, Clone)]
    struct HasIter;

    impl HasIter {
        fn iter(self) -> HasCount {
            HasCount { foo: 0 }
        }

        fn iter_mut(self) -> HasCount {
            HasCount { foo: 0 }
        }

        fn into_iter(self) -> HasCount {
            HasCount { foo: 0 }
        }
    }

    #[derive(Copy, Clone)]
    struct HasCount {
        foo: usize
    }

    impl HasCount {
        fn count(&self) {
            unimplemented!()
        }
    }

    fn no_match() {
        let false_positive = HasIter;
        false_positive.iter().count();
        false_positive.iter_mut().count();
        false_positive.into_iter().count();
    }
}
