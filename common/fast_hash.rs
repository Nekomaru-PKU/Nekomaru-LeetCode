

mod fast_hash {
    use std::{
        collections::HashMap,
        hash::Hasher,
        hash::BuildHasherDefault,
        marker::PhantomData,
    };

    #[derive(Default, Clone, Copy)]
    pub struct FastHasher<T>(u64, PhantomData<T>);

    impl Hasher for FastHasher<i32> {
        fn write_i32(&mut self, n: i32) {
            self.0 = n as _;
        }

        fn write(&mut self, _: &[u8]) {
            panic!("invalid use.")
        }

        fn finish(&self) -> u64 { self.0 }
    }

    pub type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FastHasher<K>>>;
}
