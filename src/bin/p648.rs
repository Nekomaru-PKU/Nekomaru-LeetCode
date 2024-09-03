mod trie {
    // Though public methods on [`Trie`] take [`str`] and [`usize`] for arguments,
    // [`Trie`] internally uses `u32` for node indices and `u8` for ingle
    // characters for a more compact layout and better cache locality.
    type Char = u8;
    type NodeIndex = u32;

    struct Node {
        /// The character on the node. `'\0'` if the node is the root node.
        char: Char,
        /// `true` if the string of the node is included in the dictionary.
        in_dict: bool,
        /// Index of the next sibling node.
        /// [`INDEX_NULL`] if the node is the last child of its parent.
        next_sibling: NodeIndex,
        /// Index of the first child node.
        /// [`INDEX_NULL`] if the node is a leaf node.
        first_child: NodeIndex,
    }

    /// A internal [`Trie`] node index representing a node that does not exist.
    /// 
    /// Note that every method on [`Trie`] does not accept [`INDEX_NULL`]
    /// as argument without explicit mention.
    const NOT_EXIST: NodeIndex = NodeIndex::MAX;

    impl Node {
        fn new(char: Char) -> Self {
            Self {
                char,
                in_dict: false,
                next_sibling: NOT_EXIST,
                first_child: NOT_EXIST,
            }
        }
    }

    /// A sparse tree structure to accelerate querying whether a prefix of a
    /// given word appears in a given set of strings called the *dictionary*.
    /// A [`Trie`] has at least one node.
    pub struct Trie {
        nodes: Vec<Node>,
    }

    impl Trie {
        fn with_capacity(capacity: usize) -> Self {
            Self {
                nodes: {
                    let mut nodes = Vec::with_capacity(capacity);
                    nodes.push(Node::new(0));
                    nodes
                }
            }
        }

        pub fn from_dict<S: AsRef<str>>(dict: &[S]) -> Self {
            let mut trie = Self::with_capacity(dict.iter().map(|s| s.as_ref().len()).sum());
            for word in dict.iter() {
                trie.insert_dict(word.as_ref().as_bytes());
            }
            trie
        }

        /// Finds the first sibling of `node` that has the character `char` and
        /// returns the index of the node or [`INDEX_NULL`] if not found.
        /// [`INDEX_NULL`] is invalid for `index`.
        fn find_next_sibling_with_char(&self, mut index: NodeIndex, char: Char) -> NodeIndex {
            loop {
                let node = &self.nodes[index as usize];
                if node.char == char {
                    return index;
                }
                if node.next_sibling == NOT_EXIST {
                    return NOT_EXIST;
                }
                index = node.next_sibling;
            }
        }

        /// Finds the last sibling of the node at `index` and returns its index.
        /// [`INDEX_NULL`] is invalid for `index`.
        fn find_last_sibling(&self, mut index: NodeIndex) -> NodeIndex {
            loop {
                let next = self.nodes[index as usize].next_sibling;
                if next == NOT_EXIST {
                    return index;
                }
                index = next;
            }
        }

        /// Finds the *best prefix* of `word` in the trie.
        /// This internal method is the core of a [`Trie`] and is used by
        /// other key methods on [`Trie`].
        /// 
        /// The *best prefix* of `word` is defined as the shortest prefix of
        /// `word` that is included in the dictionary.
        /// If no such prefix exists, the *best prefix* is defined as the 
        /// longest prefix of `word` that is also a prefix of some word in 
        /// the dictionary.
        /// By definition, the node for the *best prefix* of `word` is always
        /// included in the trie.
        /// 
        /// This function returns a tuple of (`idx`, `len`), where `idx`
        /// is the index of the trie node that represents the *best prefix*
        /// of `word`, and `len` is the length of the *best prefix* of `word`.
        /// 
        /// Note that the *best prefix* is included in the dictionary if
        /// and only if `self.nodes[`node`].in_dict` is `true`.
        /// 
        fn find_best_prefix(&self, word: &[Char]) -> (NodeIndex, usize) {
            // here `last_match` is the index of the last node that we've matched so far.
            // it's never `INDEX_NULL` since the empty string, represented by the root
            // node, is always a prefix of any word.
            // let's start matching `word` from the root node, which is always the node
            // at index `0`.
            let mut last_match = 0;

            // here `len` is the length of the prefix of `word` that we've matched so far
            // and `char` is the next character in `word` that we want to match.
            for (len, &char) in word.iter().enumerate() {
                let last_match_first_child = self.nodes[last_match as usize].first_child;
                if last_match_first_child == NOT_EXIST {
                    // we're on a leaf node, and we haven't matched the whole
                    // `word` yet.
                    // therefore, the *best prefix* of `word` is `word[..len]`.
                    return (last_match, len);
                }

                let this_match = self.find_next_sibling_with_char(
                    last_match_first_child,
                    char);
                if this_match == NOT_EXIST {
                    // we're done but we haven't matched the whole `word` yet.
                    // so we know that the trie contains a node for `word[..len]`,
                    // but not `word[..=len]`.
                    // therefore, the *best prefix* of `word` is `word[..len]`.
                    return (last_match, len);
                }

                // we've made a successful match at `word[len]`.
                last_match = this_match;

                // let's check if `word[..=len]` is included in the dictionary.
                if self.nodes[this_match as usize].in_dict {
                    // perfect. we've found that `word[..=len]` is in the dictionary.
                    return (this_match, len + 1);
                }

                // now we've found that the trie contains a node for `word[..=i]`,
                // but it is not included in the dictionary.
                // so let's check the next character in `word`.
            }

            // we've checked the whole `word`. it is not in the dictionary, but the
            // trie contains a node for it.
            // in this case the *best prefix* of `word` is `word` itself.
            (last_match, word.len() as _)
        }

        /// Finds the length of the longest prefix of `word` that is included in the
        /// dictionary. Returns `0` if no such prefix exists.
        pub fn find_prefix_in_dict(&self, word: &[Char]) -> usize {
            let (idx, len) = self.find_best_prefix(word);
            if self.nodes[idx as usize].in_dict {
                len
            } else {
                0
            }
        }

        fn insert_child(&mut self, parent: NodeIndex, char: Char) -> NodeIndex {
            let inserted = self.nodes.len() as NodeIndex;
            self.nodes.push(Node::new(char));

            let first_child = self.nodes[parent as usize].first_child;
            if first_child == NOT_EXIST {
                self.nodes[parent as usize].first_child = inserted;
            } else {
                let last_child = self.find_last_sibling(first_child);
                self.nodes[last_child as usize].next_sibling = inserted;
            }

            inserted
        }

        /// Insert a word from the dictionary into the trie.
        pub fn insert_dict(&mut self, word: &[Char]) {
            let (idx, len) = self.find_best_prefix(word);
            if self.nodes[idx as usize].in_dict {
                // the dictionary already contains the word.
                return;
            }

            if len == word.len() {
                // the trie already contains a node for `word`. we just need
                // to mark it as included in the dictionary.
                self.nodes[idx as usize].in_dict = true;
                return;
            }

            // the trie has a node for `word[..best_prefix_len]`, but not for
            // `word[..=best_prefix_len]`.
            let mut idx = idx;
            for &char in &word[len..] {
                idx = self.insert_child(idx, char);
            }
            self.nodes[idx as usize].in_dict = true;
        }
    }
}

fn solution(dict: Vec<String>, sentence: String) -> String {
    let trie = trie::Trie::from_dict(&dict);
    sentence.split(' ')
        .map(|word| {
            let len = trie.find_prefix_in_dict(word.as_bytes());
            if len == 0 {
                word
            } else {
                &word[..len]
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    test_example_1();
    test_example_2();
    test_perf();
}

fn test_example_1() {
    const DICT: &[&str] = &["cat","bat","rat"];
    const S_IN: &str = "the cattle was rattled by the battery";
    const S_EXPECTED: &str = "the cat was rat by the bat";

    let dict = DICT.iter().map(<_>::to_string).collect::<Vec<_>>();
    let trie = trie::Trie::from_dict(&dict);
    assert_eq!(trie.find_prefix_in_dict(b"cat"), 3);
    assert_eq!(trie.find_prefix_in_dict(b"cattle"), 3);
    assert_eq!(solution(dict, S_IN.into()), S_EXPECTED);
}

fn test_example_2() {
    const DICT: &[&str] = &["a","b","c"];
    const S_IN: &str = "aadsfasf absbs bbab cadsfafs";
    const S_EXPECTED: &str = "a a b c";

    let dict = DICT.iter().map(<_>::to_string).collect::<Vec<_>>();
    let trie = trie::Trie::from_dict(&dict);
    assert_eq!(trie.find_prefix_in_dict(b"aadsfasf"), 1);
    assert_eq!(trie.find_prefix_in_dict(b"d"), 0);
    assert_eq!(solution(dict, S_IN.into()), S_EXPECTED);
}

fn test_perf() {
    let dict = (0..1000).map(|n| "a".repeat(n) + "b").collect();
    let sentence = [].into_iter()
        .chain((0..500).map(|i| "a".repeat(500 + i) + "a"))
        .chain((0..500).map(|i| "a".repeat(500 + i) + "b"))
        .collect::<Vec<_>>()
        .join(" ");
    let expected = [].into_iter()
        .chain((0..500).map(|i| "a".repeat(500 + i) + "a"))
        .chain((0..500).map(|i| "a".repeat(500 + i) + "b"))
        .collect::<Vec<_>>()
        .join(" ");
    let result = leetcode::perf::time("perf", move || solution(dict, sentence));
    assert_eq!(result, expected);
}
