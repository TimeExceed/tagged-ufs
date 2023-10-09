# Tagged Union-Find Sets

In production use, besides testing whether two elements are in a same set,
we also often want to know the size of a set or even iteration over it.

These requirements can be abstracted as mergable tags.
That is to say, sets are associated with tags.
When two sets are united, their tags are merged.

## Use as typical union-find sets, but countable and iterable

Just associating `()` to sets.

```rust
use tagged_ufs::*;
use std::collections::BTreeSet;

let mut sets = UnionFindSets::new();
// adds 3 elements
sets.make_set(0, ());
sets.make_set(1, ());
sets.make_set(2, ());
// unites two of them
sets.unite(&0, &1);
// same-set testing
let set_0 = sets.find(&0).unwrap();
let set_1 = sets.find(&1).unwrap();
let set_2 = sets.find(&2).unwrap();
assert_eq!(set_0, set_1);
assert_ne!(set_0, set_2);
assert_ne!(set_1, set_2);
// cardinal querying
assert_eq!(set_0.len(), 2);
assert_eq!(set_2.len(), 1);
// iteration over elements in a set
assert_eq!(set_0.iter().copied().collect::<BTreeSet<_>>(), BTreeSet::from([0, 1]));
assert_eq!(set_2.iter().copied().collect::<BTreeSet<_>>(), BTreeSet::from([2]));
// iteration over sets
assert_eq!(sets.len(), 2);
let trial_sets: BTreeSet<BTreeSet<_>> = sets.iter().map(|xs| {
    xs.iter().copied().collect()
}).collect();
let oracle_sets: BTreeSet<BTreeSet<_>> = [
    BTreeSet::from([0, 1]),
    BTreeSet::from([2]),
].into_iter().collect();
assert_eq!(trial_sets, oracle_sets);
```

## Customized tags

The customized tag type must implement [Mergable].

```rust
use tagged_ufs::*;

struct Tag {
    x: usize,
}

impl Mergable for Tag {
    fn merge(&mut self, other: Tag) {
        self.x += other.x;
    }
}

let mut sets = UnionFindSets::new();
sets.make_set(0, Tag { x: 1 });
sets.make_set(1, Tag { x: 2 });
assert_eq!(sets.find(&0).unwrap().tag().x, 1);
assert_eq!(sets.find(&1).unwrap().tag().x, 2);
sets.unite(&0, &1);
assert_eq!(sets.find(&0).unwrap().tag().x, 3);
```

## Raw implementation (without element iteration)

Element iteration is also implemented by mergable tags, say, [IterableTag].
So, when it become overhead, one can bypass this layer.

```rust
use tagged_ufs::raw::*;
use std::collections::BTreeSet;

let mut sets = UnionFindSets::new();
// adds 3 elements
sets.make_set(0, ());
sets.make_set(1, ());
sets.make_set(2, ());
// unites two of them
sets.unite(&0, &1);
// same-set testing
let set_0 = sets.find(&0).unwrap();
let set_1 = sets.find(&1).unwrap();
let set_2 = sets.find(&2).unwrap();
assert_eq!(set_0, set_1);
assert_ne!(set_0, set_2);
assert_ne!(set_1, set_2);
// cardinal querying
assert_eq!(set_0.len(), 2);
assert_eq!(set_2.len(), 1);
// but iteration over sets is still okey
assert_eq!(sets.len(), 2);
let trial_set_cardinals: BTreeSet<usize> = sets.iter().map(|xs| xs.len()).collect();
assert_eq!(trial_set_cardinals, BTreeSet::from([1, 2]));
```
