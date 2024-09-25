mod trie {
    const NUM_DICTS: usize = 2;

    #[derive(Debug, Default)]
    pub struct TrieNode {
        children: [Option<Box<TrieNode>>; 10],
        is_prefix: [bool; NUM_DICTS],
    }

    pub fn insert<T: Copy + Into<usize>>(
        node: &mut Option<Box<TrieNode>>,
        s: &[T],
        dict_id: usize) -> bool {
        let node = node.get_or_insert(Box::default());
        node.is_prefix[dict_id] = 
            s.is_empty() ||
            self::insert(
                &mut node.children[s[0].into()],
                &s[1..],
                dict_id);
        node.is_prefix[dict_id]
    }

    pub fn longest_common_prefix(root: &Option<Box<TrieNode>>) -> i32 {
        self::longest_path_with(root, |node| {
            node.is_prefix == [true; NUM_DICTS]
        }) - 1
    }

    /// Finds the longest path starting from `node` and going downwards
    /// that each node on this path satisfies `pred`.
    /// 
    /// Returns the number of nodes on this path.
    fn longest_path_with(
        node: &Option<Box<TrieNode>>,
        pred: fn(&TrieNode) -> bool) -> i32 {
        node.as_ref()
            .filter(|&node| pred(node))
            .map(|node| 1 + {
                node.children
                    .iter()
                    .map(|node| self::longest_path_with(node, pred))
                    .max()
                    .unwrap()
            })
            .unwrap_or_default()
    }
}

mod int {
    const MAX_DIGITS_I32: usize = 10;

    pub fn get_digits_i32(mut num: i32) -> ([u8; MAX_DIGITS_I32], usize) {
        debug_assert!(num >= 1);
        let mut buf = [0; MAX_DIGITS_I32];
        let mut len = 0;
        while num > 0 {
            buf[len] = (num % 10).try_into().unwrap();
            num /= 10;
            len += 1;
        }
        buf[..len].reverse();
        (buf, len)
    }
}

fn solution(dict0: Vec<i32>, dict1: Vec<i32>) -> i32 {
    let mut root = None;
    for (dict_id, dict) in [(0, &dict0), (1, &dict1)] {
        for &num in dict {
            let (digits_buf, digits_len) = int::get_digits_i32(num);
            trie::insert(
                &mut root,
                &digits_buf[..digits_len],
                dict_id);
        }
    }
    trie::longest_common_prefix(&root)
}

fn main() {
    assert_eq!(solution(vec![1, 10, 100], vec![1000]), 3);
    assert_eq!(solution(vec![1, 2, 3], vec![4, 4, 4]), 0);
}
