# Tagged Union-Find Sets

In industrial use, besides testing whether two elements are in a same set,
we often want to know the size of a set, iterate over a set or do some other things about the sets.
The mergable tags are a natural way to achieve these.
That is to say, sets are associated with tags.
When two sets are united, their tags are merged.

## Recipes

### Minimal overhead union-find sets

Just use `SizedTag<()>` as tags, like the following example.

```rust
use tagged_ufs::{SizedTag, UnionFindSets};

# fn main() {
let mut ufs = UnionFindSets::<u64, SizedTag<()>>::new();
ufs.make_set(0, SizedTag::new(())).unwrap();
ufs.make_set(1, SizedTag::new(())).unwrap();
ufs.make_set(2, SizedTag::new(())).unwrap();
ufs.unite(&0, &1).unwrap();
let set_0 = ufs.find(&0).unwrap();
let set_1 = ufs.find(&1).unwrap();
let set_2 = ufs.find(&2).unwrap();
assert_eq!(ufs.len(), 2);
assert_eq!(set_0.root, set_1.root);
assert_ne!(set_0.root, set_2.root);
# }
```

### Iteration over sets

> Users usually need not implement their own iterable types.
> This crate provides `tag::SizedIterable`, which is suitable in most cases.

First, we need to define a type for tags.
In order to efficiently merge tags, we use linked lists to record the elements in the sets.

```rust
use std::collections::LinkedList;
use tagged_ufs::{Lengthed, Mergable};

struct MyIterable<Key> {
    elems: LinkedList<Key>,
}
```

Second, we implement `Mergable` and `Lengthed` for `MyIterable`.

```rust
# use std::collections::LinkedList;
# use tagged_ufs::{Lengthed, Mergable};
# struct MyIterable<Key> {
#     elems: LinkedList<Key>,
# }
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
```

Finally, we can test it.

```rust
# use std::collections::LinkedList;
# use tagged_ufs::{Lengthed, Mergable};
#
# struct MyIterable<Key> {
#     elems: LinkedList<Key>,
# }
#
# impl<Key> Lengthed for MyIterable<Key> {
#     fn len(&self) -> usize {
#         self.elems.len()
#     }
# }
#
# impl<Key> Mergable<Key> for MyIterable<Key> {
#     fn merge<K1, K2>(&mut self, mut other: Self, _key1: &K1, _key2: &K2)
#     where
#         K1: std::borrow::Borrow<Key>,
#         K2: std::borrow::Borrow<Key>,
#     {
#         self.elems.append(&mut other.elems);
#     }
# }
#
impl<Key> MyIterable<Key> {
    fn new(init: Key) -> Self {
        Self {
            elems: LinkedList::from([init]),
        }
    }
}

# fn main() {
use tagged_ufs::UnionFindSets;
let mut ufs = UnionFindSets::<u64, MyIterable<u64>>::new();
ufs.make_set(0, MyIterable::new(0)).unwrap();
ufs.make_set(1, MyIterable::new(1)).unwrap();
ufs.make_set(2, MyIterable::new(2)).unwrap();
ufs.unite(&0, &1).unwrap();
let set_0 = ufs.find(&0).unwrap();
let set_1 = ufs.find(&1).unwrap();
let set_2 = ufs.find(&2).unwrap();
println!("{:?}", set_0.tag.elems); // [0, 1] or [1, 0]
println!("{:?}", set_2.tag.elems); // [2]
# }
```

For those who want to read the complete example, please refer to [examples/my_iterable.rs](https://github.com/TimeExceed/tagged-ufs/blob/main/examples/my_iterable.rs).

### Minimal spanning tree

Please take a look at the example [examples/mst.rs](https://github.com/TimeExceed/tagged-ufs/blob/main/examples/mst.rs).
