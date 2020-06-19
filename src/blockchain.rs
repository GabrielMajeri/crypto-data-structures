use crate::HashPointer;

use std::hash::Hash;

/// Node in a linked list that uses hash pointers.
#[derive(Hash)]
pub struct Block<T: Hash> {
    ptr: Option<HashPointer<Self>>,
    data: T,
}

impl<T: Hash> Block<T> {
    /// Allocates a new block, with nothing after it.
    pub fn new(data: T) -> Self {
        Self { ptr: None, data }
    }

    /// Inserts a new value before the chain.
    pub fn prepend(self, data: T) -> Self {
        Self { ptr: Some(HashPointer::new(self)), data }
    }

    /// Iterator providing a read-only view through this block chain.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BlockIterator { block: Some(&self) }
    }
}

struct BlockIterator<'a, T: Hash + 'a> {
    block: Option<&'a Block<T>>,
}

impl<'a, T: Hash> Iterator for BlockIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.block.map(|Block { ptr, data }| {
            self.block = ptr.as_deref();
            data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_chain_and_iterate() {
        let mut chain = Block::<i32>::new(0);
        chain = chain.prepend(7);
        chain = chain.prepend(5);
        chain = chain.prepend(11);
        chain = chain.prepend(-42);
        let values: Vec<_> = chain.iter().cloned().collect();
        assert_eq!(&values[..], [-42, 11, 5, 7, 0]);
    }
}
