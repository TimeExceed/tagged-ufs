use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Mergable<Key> {
    fn merge<K1, K2>(&mut self, other: Self, key1: &K1, key2: &K2)
    where
        K1: Borrow<Key>,
        K2: Borrow<Key>;
}

pub trait Lengthed {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Raw implementation of union-find sets, with built-in balanced union and path compression.
#[derive(Clone)]
pub struct UnionFindSets<Key, Tag>
where
    Key: Eq + Hash,
    Tag: Mergable<Key> + Lengthed,
{
    parents: RefCell<HashMap<Key, Key, ahash::RandomState>>,
    tags: HashMap<Key, Tag, ahash::RandomState>,
}

impl<Key, Tag> Default for UnionFindSets<Key, Tag>
where
    Key: Eq + Hash + Clone,
    Tag: Mergable<Key> + Lengthed,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Key, Tag> UnionFindSets<Key, Tag>
where
    Key: Eq + Hash + Clone,
    Tag: Mergable<Key> + Lengthed,
{
    /// Makes a new, empty set of sets.
    pub fn new() -> Self {
        Self {
            parents: RefCell::new(HashMap::with_hasher(ahash::RandomState::new())),
            tags: HashMap::with_hasher(ahash::RandomState::new()),
        }
    }

    /// Makes an individual set with a singleton element and its associated tag.
    ///
    /// If the set to make is already there,
    /// an error will be raised and nothing will happen to the sets.
    pub fn make_set(&mut self, key: Key, tag: Tag) -> anyhow::Result<()> {
        {
            let parents = self.parents.borrow();
            if parents.contains_key(&key) {
                anyhow::bail!("Duplicated key!");
            }
        }
        if self.tags.contains_key(&key) {
            anyhow::bail!("Duplicated key!");
        }
        self.tags.insert(key, tag);
        Ok(())
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
        let Some(key1_top) = self.find_root_key(key1) else {
            anyhow::bail!("Cannot find set: {:?}", key1);
        };
        let Some(key2_top) = self.find_root_key(key2) else {
            anyhow::bail!("Cannot find set: {:?}", key2);
        };
        if key1_top == key2_top {
            return Ok(false);
        }
        let key1_top = key1_top.clone();
        let key2_top = key2_top.clone();
        let mut key1_tag = self.tags.remove(&key1_top).unwrap();
        let mut key2_tag = self.tags.remove(&key2_top).unwrap();
        let key1_is_parent = key1_tag.len() > key2_tag.len();
        let mut parents = self.parents.borrow_mut();
        if key1_is_parent {
            key1_tag.merge(key2_tag, key1, key2);
            parents.insert(key2_top, key1_top.clone());
            self.tags.insert(key1_top, key1_tag);
        } else {
            key2_tag.merge(key1_tag, key2, key1);
            parents.insert(key1_top, key2_top.clone());
            self.tags.insert(key2_top, key2_tag);
        }
        Ok(true)
    }

    /// Finds an individual set.
    ///
    /// If the set is not inside, `None` will be returned.
    pub fn find<K>(&self, key: &K) -> Option<Set<Key, Tag>>
    where
        K: Eq + Hash + Borrow<Key>,
    {
        let root_key = self.find_root_key(key)?;
        let tag = self.tags.get(root_key).unwrap();
        Some(Set {
            root: root_key,
            tag,
        })
    }

    /// Iterates over all individual sets.
    pub fn iter(&self) -> impl Iterator<Item = Set<Key, Tag>> {
        self.tags.iter().map(|(key, tag)| Set { root: key, tag })
    }

    /// Queries the number of individual sets in the set.
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Tests if this set (of sets) is empty.
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    fn find_root_key<K>(&self, key: &K) -> Option<&Key>
    where
        K: Hash + Eq + Borrow<Key>,
    {
        self.find_top_key_(key.borrow())
    }

    fn find_top_key_(&self, key: &Key) -> Option<&Key> {
        let mut keys = vec![];
        let top = {
            let parents = self.parents.borrow();
            self.collect_keys(key, &mut keys, parents.borrow())?
        };
        keys.pop();
        if !keys.is_empty() {
            let mut parents = self.parents.borrow_mut();
            while let Some(mid_key) = keys.pop() {
                parents.insert(mid_key, top.clone());
            }
        }
        Some(top)
    }

    fn collect_keys(
        &self,
        key: &Key,
        keys: &mut Vec<Key>,
        parents: &HashMap<Key, Key, ahash::RandomState>,
    ) -> Option<&Key> {
        if let Some(nxt_key) = parents.get(key) {
            keys.push(key.clone());
            self.collect_keys(nxt_key, keys, parents)
        } else if let Some((top, _)) = self.tags.get_key_value(key) {
            Some(top)
        } else {
            None
        }
    }
}

/// An individual set of elements,
/// which is able to neither inspect into nor iterate over its elements.
#[derive(Debug)]
pub struct Set<'a, Key, Tag>
where
    Key: Eq,
{
    pub root: &'a Key,
    pub tag: &'a Tag,
}

impl<'a, Key: Eq + Hash, Tag> PartialEq for Set<'a, Key, Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.root.eq(other.root)
    }
}

impl<'a, Key: Eq + Hash, Tag> Eq for Set<'a, Key, Tag> {}
