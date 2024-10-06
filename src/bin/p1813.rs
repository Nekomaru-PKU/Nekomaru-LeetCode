mod solution {
    pub fn main(s0: String, s1: String) -> bool {
        #![expect(clippy::suspicious_operation_groupings)]

        let s0 = s0.as_bytes();
        let s1 = s1.as_bytes();

        let mut tokenizer = Tokenizer::new();
        let s0 = tokenizer.tokenize(s0);
        let s1 = tokenizer.tokenize(s1);

        let mut s0 = s0.as_slice();
        let mut s1 = s1.as_slice();
        while
            !s0.is_empty() &&
            !s1.is_empty() &&
            s0[0] == s1[0] {
            s0 = &s0[1..];
            s1 = &s1[1..];
        }
        while
            !s0.is_empty() &&
            !s1.is_empty() &&
            s0[s0.len() - 1] == s1[s1.len() - 1] {
            s0 = &s0[..(s0.len() - 1)];
            s1 = &s1[..(s1.len() - 1)];
        }
        s0.is_empty() || s1.is_empty()
    }

    use std::collections::{
        HashMap,
        hash_map::Entry,
    };

    struct Tokenizer<'a> {
        tokens: HashMap<&'a [u8], u8>,
        next_id: u8,
    }

    impl<'a> Tokenizer<'a> {
        const MAX_TOKENS: usize = 52;

        fn new() -> Self {
            Self {
                tokens: HashMap::with_capacity(Self::MAX_TOKENS),
                next_id: 0,
            }
        }

        fn tokenize(&mut self, s: &'a [u8]) -> Vec<u8> {
            let mut range = 0..0;
            let mut out = Vec::with_capacity(Self::MAX_TOKENS);
            for c in s.iter().copied().chain(Some(b' ')) {
                if c == b' ' {
                    let token = &s[range.clone()];
                    if !token.is_empty() {
                        match self.tokens.entry(token) {
                            Entry::Occupied(entry) => out.push(*entry.get()),
                            Entry::Vacant(entry) => {
                                let token_id = self.next_id;
                                self.next_id += 1;
                                entry.insert(token_id);
                                out.push(token_id);
                            }
                        };
                    }
                    range.end += 1;
                    range.start = range.end;
                } else {
                    range.end += 1;
                }
            }
            out
        }
    }
}

fn main() {
    assert!(solution::main("My name is Haley".into(), "My Haley".into()));
    assert!(!solution::main("of".into(), "A lot of words".into()));
    assert!(solution::main("Eating right now".into(), "Eating".into()));
}
