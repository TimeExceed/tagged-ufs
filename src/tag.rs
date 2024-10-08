use crate::{Lengthed, Mergable};
use std::{
    borrow::Borrow,
    collections::{linked_list, LinkedList},
};

impl<Key> Mergable<Key> for () {
    fn merge<K1, K2>(&mut self, _: Self, _: &K1, _: &K2)
    where
        K1: Borrow<Key>,
        K2: Borrow<Key>,
    {
    }
}

/// A wrapper to customized tag, which can inspect the size of a set.
#[derive(Debug, Clone)]
pub struct SizedTag<Tag> {
    size: usize,
    tag: Tag,
}

impl<T> SizedTag<T> {
    pub fn new(tag: T) -> Self {
        Self { size: 1, tag }
    }
}

impl<T> Lengthed for SizedTag<T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<Key, T> Mergable<Key> for SizedTag<T>
where
    T: Mergable<Key>,
{
    fn merge<K1, K2>(&mut self, other: Self, key1: &K1, key2: &K2)
    where
        K1: Borrow<Key>,
        K2: Borrow<Key>,
    {
        self.size += other.size;
        self.tag.merge(other.tag, key1, key2);
    }
}

/// A wrapper to customized tag, which provides iterability over elements.
///
/// The iterability is implemented by linked list.
/// So, merging two SizedIterable's has O(1) overhead.
#[derive(Debug, Clone)]
pub struct SizedIterable<Key, Tag> {
    sets: LinkedList<Key>,
    tag: Tag,
}

impl<Key, Tag> Lengthed for SizedIterable<Key, Tag> {
    fn len(&self) -> usize {
        self.sets.len()
    }
}

impl<Key, Tag> Mergable<Key> for SizedIterable<Key, Tag>
where
    Tag: Mergable<Key>,
{
    fn merge<K1, K2>(&mut self, mut other: Self, key1: &K1, key2: &K2)
    where
        K1: Borrow<Key>,
        K2: Borrow<Key>,
    {
        self.sets.append(&mut other.sets);
        self.tag.merge(other.tag, key1, key2);
    }
}

impl<Key, Tag> SizedIterable<Key, Tag> {
    pub fn new(key: Key, tag: Tag) -> Self {
        Self {
            sets: LinkedList::from_iter([key]),
            tag,
        }
    }

    pub fn iter(&self) -> linked_list::Iter<'_, Key> {
        self.sets.iter()
    }

    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    pub fn into_inner(self) -> (LinkedList<Key>, Tag) {
        (self.sets, self.tag)
    }
}

impl<Key, Tag> IntoIterator for SizedIterable<Key, Tag> {
    type Item = Key;
    type IntoIter = linked_list::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.sets.into_iter()
    }
}
