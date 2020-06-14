use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Hashes a block of data using the default hasher.
fn hash<T: Hash>(data: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

/// Stores a pointer and the hash of some data.
///
/// If the computed hash of the data is different from the stored value,
/// one can deduce the data has been modified.
pub struct HashPointer<T: Hash> {
    hash: u64,
    data: Box<T>,
}

impl<T: Hash> HashPointer<T> {
    pub fn new(data: T) -> Self {
        Self::from_box(Box::new(data))
    }

    pub fn from_box(data: Box<T>) -> Self {
        let hash = hash(&data);
        Self { hash, data }
    }

    /// Returns `true` if the hash of the data is the same as
    /// the hash value initially computed.
    pub fn validate(&self) -> bool {
        self.hash == hash(&self.data)
    }
}

impl<T: Hash> AsRef<T> for HashPointer<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T: Hash> AsMut<T> for HashPointer<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_data() {
        let hptr = HashPointer::new("hello");
        assert!(hptr.validate());
    }

    #[test]
    fn from_box() {
        let x = Box::new("world");
        let hptr = HashPointer::from_box(x);
        assert!(hptr.validate());
    }

    #[test]
    fn detect_mutation() {
        let mut hptr = HashPointer::new("hello");
        *hptr.as_mut() = "abcd";
        assert!(!hptr.validate());
    }

    #[test]
    fn same_data_after_mutation() {
        let mut hptr = HashPointer::new("same");
        *hptr.as_mut() = "same";
        assert!(hptr.validate());
    }
}
