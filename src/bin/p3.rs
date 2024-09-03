mod bitvec {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct BitVec256([u64; 4]);

    impl BitVec256 {
        pub const fn new() -> Self { Self([0; 4]) }

        pub const fn get(&self, i: u8) -> bool {
            self.0[(i >> 6) as usize] & (1 << (i & 0x3F)) > 0
        }

        pub fn set(&mut self, i: u8) {
            self.0[(i >> 6) as usize] |= 1 << (i & 0x3F);
        }

        pub fn reset(&mut self, i: u8) {
            self.0[(i >> 6) as usize] &= !(1 << (i & 0x3F));
        }
    }
}

fn solution(s: String) -> i32 {
    use bitvec::BitVec256;
    let s = s.into_bytes();
    let mut begin = 0;
    let mut end = 0;
    let mut max = 0;
    let mut chars = BitVec256::new();
    while begin < s.len() {
        while end < s.len() && !chars.get(s[end]) {
            chars.set(s[end]);
            end += 1;
        }
        max = max.max(end - begin);
        chars.reset(s[begin]);
        begin += 1;
    }
    max as _
}

fn main() {
    assert_eq!(solution("abcabcbb".into()), 3);
    assert_eq!(solution("bbbbb".into()), 1);
    assert_eq!(solution("pwwkew".into()), 3);
    assert_eq!(solution(String::new()), 0);
}
