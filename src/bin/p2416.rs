mod trie {
    #[derive(Debug, Default)]
    pub struct TrieNode {
        pub children: [Option<Box<TrieNode>>; 26],
        pub num_terminals_in_subtree: i32,
    }

    pub fn insert<S: Iterator<Item = u8>>(
        root: &mut Option<Box<TrieNode>>,
        mut iter: S) {
        let mut next = root;
        while let Some(c) = iter.next() {
            let node = next.get_or_insert(Box::default());
            node.num_terminals_in_subtree += 1;
            next = &mut node.children[(c - b'a') as usize];
        }
        next.get_or_insert(Box::default())
            .num_terminals_in_subtree += 1;
    }
}

pub fn solution(words: Vec<String>) -> Vec<i32> {
    let mut root = None;
    for word in &words {
        trie::insert(&mut root, word.bytes());
    }
    words.iter().map(|word| {
        let mut iter = word.bytes();
        let mut node = root.as_ref().unwrap().as_ref();
        let mut acc = 0;
        while let Some(c) = iter.next() {
            node = &node.children[(c - b'a') as usize].as_ref().unwrap();
            acc  += node.num_terminals_in_subtree;
        }
        acc
    }).collect()
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution(["abc", "ab", "bc", "b"].input()), [5, 4, 3, 2]);
    assert_eq!(solution(["abcd"].input()), [4]);
}
