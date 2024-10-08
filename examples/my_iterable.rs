use std::collections::LinkedList;
use tagged_ufs::{Lengthed, Mergable, UnionFindSets};

struct MyIterable<Key> {
    elems: LinkedList<Key>,
}

impl<Key> Lengthed for MyIterable<Key> {
    fn len(&self) -> usize {
        self.elems.len()
    }
}

impl<Key> Mergable<Key> for MyIterable<Key> {
    fn merge<K1, K2>(&mut self, mut other: Self, _key1: &K1, _key2: &K2)
    where
        K1: std::borrow::Borrow<Key>,
        K2: std::borrow::Borrow<Key>,
    {
        self.elems.append(&mut other.elems);
    }
}

impl<Key> MyIterable<Key> {
    fn new(init: Key) -> Self {
        Self {
            elems: LinkedList::from([init]),
        }
    }
}

fn main() {
    let mut ufs = UnionFindSets::<u64, MyIterable<u64>>::new();
    ufs.make_set(0, MyIterable::new(0)).unwrap();
    ufs.make_set(1, MyIterable::new(1)).unwrap();
    ufs.make_set(2, MyIterable::new(2)).unwrap();
    ufs.unite(&0, &1).unwrap();
    let set_0 = ufs.find(&0).unwrap();
    let set_1 = ufs.find(&1).unwrap();
    let set_2 = ufs.find(&2).unwrap();
    assert_eq!(set_0.root, set_1.root);
    assert_ne!(set_0.root, set_2.root);
    println!("{:?}", set_0.tag.elems); // [0, 1] or [1, 0]
    println!("{:?}", set_2.tag.elems); // [2]
}
