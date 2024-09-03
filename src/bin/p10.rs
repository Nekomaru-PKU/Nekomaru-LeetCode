mod solution {
    enum Token {
        End,
        One(u8),
        OneAny,
        ZeroOrMore(u8),
        ZeroOrMoreAny,
    }

    fn tokenize(p: &[u8]) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(p.len() + 1);
        for &c in p {
            match c {
                b'a'..=b'z' =>
                    tokens.push(Token::One(c)),
                b'.' =>
                    tokens.push(Token::OneAny),
                b'*' => match tokens.pop().unwrap() {
                    Token::One(c) |
                    Token::ZeroOrMore(c) =>
                        tokens.push(Token::ZeroOrMore(c)),
                    Token::OneAny |
                    Token::ZeroOrMoreAny =>
                        tokens.push(Token::ZeroOrMoreAny),
                    Token::End => panic!(),
                },
                _ => panic!("unexpected character"),
            }
        }
        tokens.push(Token::End);
        tokens
    }

    pub fn main(s: String, p: String) -> bool {
        let s = s.into_bytes();
        let p = tokenize(&p.into_bytes());

        fn is_match(s: &[u8], p: &[Token]) -> bool {
            match p.first() {
                Some(&Token::One(c)) =>
                    s.first() == Some(&c) &&
                    is_match(&s[1..], &p[1..]),
                Some(&Token::OneAny) =>
                    !s.is_empty() &&
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

        is_match(&s, &p)
    }

    pub fn main_opt(s: String, p: String) -> bool {
        let s = s.into_bytes();
        let p = tokenize(&p.into_bytes());

        fn is_match(
            s_vec: &[u8],
            s_begin: usize,
            p_vec: &[Token],
            p_begin: usize,
            cache: &mut [[Option<bool>; 23]; 23]) -> bool {
            if let Some(value) = cache[s_begin][p_begin] {
                return value;
            }

            let s = &s_vec[s_begin..];
            let p = &p_vec[p_begin..];
            let value = match p.first() {
                Some(&Token::One(c)) =>
                    s.first() == Some(&c) &&
                    is_match(
                        s_vec,
                        s_begin + 1,
                        p_vec,
                        p_begin + 1,
                        cache),
                Some(&Token::OneAny) =>
                    !s.is_empty() &&
                    is_match(
                        s_vec,
                        s_begin + 1,
                        p_vec,
                        p_begin + 1,
                        cache),
                Some(&Token::ZeroOrMore(c)) => 
                    is_match(
                        s_vec,
                        s_begin,
                        p_vec,
                        p_begin + 1,
                        cache) ||
                    (1..=s.len())
                        .take_while(|&begin| s[begin - 1] == c)
                        .map(|begin| is_match(
                            s_vec,
                            s_begin + begin,
                            p_vec,
                            p_begin + 1,
                            cache))
                        .any(|b| b),
                Some(&Token::ZeroOrMoreAny) =>
                    (0..=s.len())
                        .map(|begin| is_match(
                            s_vec,
                            s_begin + begin,
                            p_vec,
                            p_begin + 1,
                            cache))
                        .any(|b| b),
                Some(&Token::End) => s.is_empty(),
                None => false,
            };
            cache[s_begin][p_begin] = Some(value);
            value
        }

        let mut cache = [[None; 23]; 23];
        is_match(&s, 0, &p, 0, &mut cache)
    }
}

fn main() {
    for solution in [
        solution::main,
        solution::main_opt] {
        assert!(!solution("aa".into(), "a".into()));
        assert!(solution("aa".into(), "a*".into()));
        assert!(solution("ab".into(), ".*".into()));

        assert!(solution(String::new(), String::new()));
        assert!(solution(String::new(), "a*".into()));
        assert!(solution(String::new(), ".*".into()));

        assert!(solution("aab".into(), "c*a*b".into()));
        assert!(solution("aab".into(), "c***a*b".into()));

        assert!(!solution("a".into(), ".*..a*".into()));
    }
}
