use crate::Mergable;
use std::borrow::Borrow;
use std::collections::LinkedList;
use std::hash::Hash;

/// A set of union-find sets, each of which can be associated with a mergable tag.
#[derive(Clone)]
pub struct UnionFindSets<Key, Tag>
where
    Key: Eq + Hash,
    Tag: Mergable,
{
    raw: crate::raw::UnionFindSets<Key, IterableTag<Key, Tag>>,
}

impl<Key, Tag> UnionFindSets<Key, Tag>
where
    Key: Eq + Hash + Clone,
    Tag: Mergable,
{
    /// Makes a new, empty set of sets.
    pub fn new() -> Self {
        Self {
            raw: crate::raw::UnionFindSets::new(),
        }
    }

    /// Makes an individual set with a singleton element and its associated tag.
    ///
    /// If the set to make is already there,
    /// an error will be raised and nothing will happen to the sets.
    pub fn make_set(&mut self, key: Key, tag: Tag) -> anyhow::Result<()> {
        self.raw.make_set(key.clone(), IterableTag::new(key, tag))
    }

    /// Unites two sets.
    ///
    /// If either of them is not in the sets, an error will be raised;
    /// if they are of a same set, `Ok(false)` will be returns;
    /// otherwise, which means these two sets are really united into one in this case,
    /// `Ok(true)` will be returned.
    pub fn unite<K1, K2>(&mut self, key1: &K1, key2: &K2) -> anyhow::Result<bool>
    where
        K1: Hash + Eq + Borrow<Key> + std::fmt::Debug,
        K2: Hash + Eq + Borrow<Key> + std::fmt::Debug,
    {
        self.raw.unite(key1, key2)
    }

    /// Finds an individual set.
    ///
    /// If the set is not inside, `None` will be returned.
    pub fn find<K>(&self, key: &K) -> Option<Set<Key, Tag>>
    where
        K: Eq + Hash + Borrow<Key>,
    {
        self.raw.find(key).map(|x| Set { raw: x })
    }

    /// Iterates over all individual sets.
    pub fn iter(&self) -> impl Iterator<Item = Set<Key, Tag>> {
        self.raw.iter().map(|raw| Set { raw })
    }

    /// Queries the number of individual sets in the set.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Tests if this set (of sets) is empty.
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
}

impl<Key, Tag> Default for UnionFindSets<Key, Tag>
where
    Key: Eq + Hash + Clone,
    Tag: Mergable,
{
    fn default() -> Self {
        Self::new()
    }
}

/// A wrapper to customized tag, which provides iterability over elements.
///
/// The iterability is implemented by linked list.
/// So, merging two IterableTag's has O(1) overhead.
#[derive(Debug, Clone)]
pub struct IterableTag<Key, Tag> {
    sets: LinkedList<Key>,
    tag: Tag,
}

impl<Key, Tag> Mergable for IterableTag<Key, Tag>
where
    Tag: Mergable,
{
    fn merge(&mut self, mut other: Self) {
        self.sets.append(&mut other.sets);
        self.tag.merge(other.tag);
    }
}

impl<Key, Tag> IterableTag<Key, Tag> {
    pub fn new(key: Key, tag: Tag) -> Self {
        Self {
            sets: LinkedList::from_iter([key]),
            tag,
        }
    }
}

/// An individual set
#[derive(Debug)]
pub struct Set<'a, Key, Tag>
where
    Key: Eq,
{
    raw: crate::raw::Set<'a, Key, IterableTag<Key, Tag>>,
}

impl<'a, Key: Eq + Hash, Tag> PartialEq for Set<'a, Key, Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.raw.eq(&other.raw)
    }
}

impl<'a, Key: Eq + Hash, Tag> Eq for Set<'a, Key, Tag> {}

impl<'a, Key, Tag> Set<'a, Key, Tag>
where
    Key: Eq + Hash,
    Tag: Mergable,
{
    /// Queries the number of elements in the set.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Tests if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterates over elements in the set.
    pub fn iter(&self) -> impl Iterator<Item = &Key> {
        self.raw.tag().sets.iter()
    }

    /// Gets the representative element
    pub fn key(&self) -> &Key {
        self.raw.key()
    }

    /// Gets the tag associated with this set.
    pub fn tag(&self) -> &Tag {
        &self.raw.tag().tag
    }
}
