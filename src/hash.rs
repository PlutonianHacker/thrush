use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{self, Hasher},
};

// implement custom hasher?
#[derive(PartialEq, Clone, Copy, hash::Hash)]
pub struct Hash(u64);

impl Hash {
    pub fn of<H: hash::Hash>(value: H) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        Self(hasher.finish())
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Hash(0x{})", self.0))
    }
}

#[cfg(test)]
mod test {
    use super::Hash;

    #[test]
    fn test_fn_hash() {
        assert_eq!(Hash::of("name"), Hash::of("name"));
        assert_ne!(Hash::of("a"), Hash::of("b"));
    }
}
