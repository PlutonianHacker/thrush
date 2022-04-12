use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{self, Hasher},
};

// implement custom hasher?
#[derive(PartialEq, Clone, Copy)]
pub struct Hash(u64);

impl Hash {
    pub fn of<H: hash::Hash>(value: H) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        Self(hasher.finish())
    }
}

impl hash::Hash for Hash {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
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
        println!("{:?}", Hash::of("Hello"));
        println!("{:?}", Hash::of("+"));
        println!("{:?}", Hash::of("hello_world"));
        println!("{:?}", Hash::of(123));

        assert_eq!(Hash::of("name"), Hash::of("name"));

        let hash = Hash::of("pineapple");

        
    }
}
