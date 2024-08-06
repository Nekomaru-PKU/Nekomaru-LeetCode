fn solution(s: String, p: String) -> bool {
    let s = s.into_bytes();
    let p = p.into_bytes();

    enum Token {
        End,
        One(u8),
        OneAny,
        ZeroOrMore(u8),
        ZeroOrMoreAny,
    }

    let mut p_tokens = Vec::with_capacity(p.len() + 1);
    for &c in &p {
        match c {
            b'a'..=b'z' =>
                p_tokens.push(Token::One(c)),
            b'.' =>
                p_tokens.push(Token::OneAny),
            b'*' => match p_tokens.pop().unwrap() {
                Token::One(c) |
                Token::ZeroOrMore(c) =>
                    p_tokens.push(Token::ZeroOrMore(c)),
                Token::OneAny |
                Token::ZeroOrMoreAny =>
                    p_tokens.push(Token::ZeroOrMoreAny),
                Token::End => unreachable!(),
            },
            _ => panic!("unexpected character"),
        }
    }
    p_tokens.push(Token::End);

    fn is_match(s: &[u8], p: &[Token]) -> bool {
        match p.first() {
            Some(&Token::One(c)) =>
                s.first() == Some(&c) &&
                is_match(&s[1..], &p[1..]),
            Some(&Token::OneAny) =>
                s.len() >= 1 &&
                is_match(&s[1..], &p[1..]),
            Some(&Token::ZeroOrMore(c)) => 
                is_match(s, &p[1..]) ||
                (1..=s.len())
                    .take_while(|&begin| s[begin - 1] == c)
                    .map(|begin| is_match(&s[begin..], &p[1..]))
                    .any(|b| b),
            Some(&Token::ZeroOrMoreAny) =>
                (0..=s.len())
                    .map(|begin| is_match(&s[begin..], &p[1..]))
                    .any(|b| b),
            Some(&Token::End) => s.is_empty(),
            None => false,
        }
    }

    is_match(&s, &p_tokens)
}

fn main() {
    assert!(!solution("aa".into(), "a".into()));
    assert!(solution("aa".into(), "a*".into()));
    assert!(solution("ab".into(), ".*".into()));

    assert!(solution("".into(), "".into()));
    assert!(solution("".into(), "a*".into()));
    assert!(solution("".into(), ".*".into()));

    assert!(solution("aab".into(), "c*a*b".into()));
    assert!(solution("aab".into(), "c***a*b".into()));

    assert!(!solution("a".into(), ".*..a*".into()));
}
