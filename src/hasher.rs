use nwn_files::types::ResRef;

use std::convert::TryFrom;
use std::hash::{Hash, Hasher as TraitHasher};
use std::collections::hash_map::DefaultHasher;

pub struct Hasher(DefaultHasher);

impl Hasher {
    pub fn new() -> Self {
        Hasher(DefaultHasher::new())
    }
    
    pub fn hash<T: Hash>(&mut self, val: T)
        -> &mut Self
    {
        val.hash(&mut self.0);
        self
    }

    pub fn finish(&mut self) -> u64 {
        self.0.finish()
    }

    pub fn to_res_ref(&mut self) -> ResRef {
        let hash = self.finish();
        ResRef::try_from(hash.to_string()[..16].to_string()).unwrap()
    }
}