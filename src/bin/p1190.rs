mod solution {
    pub fn main(s: String) -> String {
        let mut stack = vec![Vec::new()];
        for c in s.into_bytes() {
            match c {
                b'(' => stack.push(Vec::new()),
                b')' => {
                    let part = stack.pop().unwrap();
                    stack.last_mut().unwrap().extend(part.into_iter().rev());
                },
                _ => stack.last_mut().unwrap().push(c),
            }
        }
        String::from_utf8(stack.into_iter().next().unwrap()).unwrap()
    }
}

fn main() {
    assert_eq!(solution::main("(abcd)".into()), "dcba");
    assert_eq!(solution::main("(u(love)i)".into()), "iloveu");
    assert_eq!(solution::main("(ed(et(oc))el)".into()), "leetcode");

    assert_eq!(solution::main("abcd".into()), "abcd");
    assert_eq!(solution::main("ab(ed(et(oc))el)cd".into()), "ableetcodecd");
}
