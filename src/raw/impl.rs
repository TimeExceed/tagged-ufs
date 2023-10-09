use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Mergable {
    fn merge(&mut self, other: Self);
}

impl Mergable for () {
    fn merge(&mut self, _other: Self) {}
}

#[derive(Debug, Clone)]
pub(crate) struct SizedTag<Tag> {
    size: usize,
    tag: Tag,
}

impl<T> SizedTag<T> {
    fn new(tag: T) -> Self {
        Self { size: 1, tag }
    }
}

impl<T: Mergable> Mergable for SizedTag<T> {
    fn merge(&mut self, other: Self) {
        self.size += other.size;
        self.tag.merge(other.tag);
    }
}

/// Raw implementation of union-find sets, with built-in balanced union and path compression.
#[derive(Clone)]
pub struct UnionFindSets<Key, Tag>
where
    Key: Eq + Hash,
    Tag: Mergable,
{
    parents: RefCell<HashMap<Key, Key, ahash::RandomState>>,
    tags: HashMap<Key, SizedTag<Tag>, ahash::RandomState>,
}

/// An individual set (of elements) without the ability to iterate over elements.
#[derive(Debug)]
pub struct Set<'a, Key, Tag>
where
    Key: Eq,
{
    pub(crate) key: &'a Key,
    pub(crate) tag: &'a SizedTag<Tag>,
}

impl<'a, Key: Eq + Hash, Tag> PartialEq for Set<'a, Key, Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(other.key)
    }
}

impl<'a, Key: Eq + Hash, Tag> Eq for Set<'a, Key, Tag> {}

impl<'a, Key, Tag> Set<'a, Key, Tag>
where
    Key: Eq + Hash,
    Tag: Mergable,
{
    /// Queries the number of elements in this set.
    pub fn len(&self) -> usize {
        self.tag.size
    }

    /// Tests if this set is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the representative element
    pub fn key(&self) -> &Key {
        self.key
    }

    /// Gets the customized tag associated with this set.
    pub fn tag(&self) -> &Tag {
        &self.tag.tag
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

impl<Key, Tag> UnionFindSets<Key, Tag>
where
    Key: Eq + Hash + Clone,
    Tag: Mergable,
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
        self.tags.insert(key, SizedTag::new(tag));
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
        let Some(key1_top) = self.find_top_key(key1) else {
            anyhow::bail!("Cannot find set: {:?}", key1);
        };
        let Some(key2_top) = self.find_top_key(key2) else {
            anyhow::bail!("Cannot find set: {:?}", key2);
        };
        if key1_top == key2_top {
            return Ok(false);
        }
        let key1_top = key1_top.clone();
        let key2_top = key2_top.clone();
        let mut key1_tag = self.tags.remove(&key1_top).unwrap();
        let mut key2_tag = self.tags.remove(&key2_top).unwrap();
        let parent_key1 = key1_tag.size > key2_tag.size;
        let mut parents = self.parents.borrow_mut();
        if parent_key1 {
            key1_tag.merge(key2_tag);
            parents.insert(key2_top, key1_top.clone());
            self.tags.insert(key1_top, key1_tag);
        } else {
            key2_tag.merge(key1_tag);
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
        let key_top = self.find_top_key(key)?;
        let tag = self.tags.get(key_top).unwrap();
        Some(Set { key: key_top, tag })
    }

    /// Iterates over all individual sets.
    pub fn iter(&self) -> impl Iterator<Item = Set<Key, Tag>> {
        self.tags.iter().map(|(key, tag)| Set { key, tag })
    }

    /// Queries the number of individual sets in the set.
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Tests if this set (of sets) is empty.
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    fn find_top_key<K>(&self, key: &K) -> Option<&Key>
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
