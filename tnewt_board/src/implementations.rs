pub enum Implementation {
    Original,
    MutPass,
    HashSet,
}

pub mod original;
pub mod mut_pass;
pub mod hash_set;
