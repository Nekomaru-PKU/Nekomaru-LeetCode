mod solution {
    use std::{
        collections::HashMap,
        cmp::Ordering,
        fmt,
        fmt::Write,
        ops::*,
    };

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum AtomName {
        Short((u8, u8)),
        Long(Vec<u8>),
    }

    impl Ord for AtomName {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (&Self::Short(ref s0), &Self::Short(ref s1)) =>
                    <(u8, u8)>::cmp(s0, s1),
                (&Self::Short(ref s0), &Self::Long(ref s1)) =>
                    <[u8]>::cmp(&[s0.0, s0.1], s1),
                (&Self::Long(ref s0), &Self::Short(ref s1)) =>
                    <[u8]>::cmp(s0, &[s1.0, s1.1]),
                (&Self::Long(ref s0), &Self::Long(ref s1)) =>
                    <[u8]>::cmp(s0, s1),
            }
        }
    }

    impl PartialOrd for AtomName {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl AtomName {
        const fn with_char(c: u8) -> Self {
            Self::Short((c, 0))
        }

        fn push_char(&mut self, c: u8) {
            match *self {
                Self::Short(ref mut chars@(0, 0)) =>
                    chars.0 = c,
                Self::Short(ref mut chars@(_, 0)) =>
                    chars.1 = c,
                Self::Short((c0, c1)) =>
                    *self = Self::Long(vec![c0, c1, c]),
                Self::Long(ref mut chars) =>
                    chars.push(c),
            }
        }
    }

    impl fmt::Display for AtomName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Self::Short((0, 0)) => (),
                Self::Short((c0, 0)) =>
                    f.write_char(c0 as _)?,
                Self::Short((c0, c1)) => {
                    f.write_char(c0 as _)?;
                    f.write_char(c1 as _)?;
                },
                Self::Long(ref chars) => for &c in chars {
                    f.write_char(c as _)?;
                },
            }
            Ok(())
        }
    }

    enum Term {
        Atom(AtomName),
        Compound(Vec<(Term, u32)>),
    }

    enum TermOrPush {
        Term(Term, Option<u32>),
        Push,
    }

    pub fn main(formula: String) -> String {
        let s = formula.into_bytes();

        let mut stack = Vec::new();
        for &c in [b"(" as &[u8], &s, b")"]
            .into_iter()
            .flatten() {
            match (c, stack.last_mut()) {
                (b'A'..=b'Z', _) =>
                    stack.push(TermOrPush::Term(
                        Term::Atom(AtomName::with_char(c)),
                        None)),
                (b'a'..=b'z', Some(&mut TermOrPush::Term(
                    Term::Atom(ref mut atom),
                    _))) =>
                    atom.push_char(c),
                (b'0'..=b'9', Some(&mut TermOrPush::Term(
                    _,
                    ref mut repeat))) =>
                    if let &mut Some(ref mut repeat) = repeat {
                        *repeat *= 10;
                        *repeat += (c - b'0') as u32;
                    } else {
                        *repeat = Some((c - b'0') as _);
                    },
                (b'(', _) => stack.push(TermOrPush::Push),
                (b')', _) => {
                    let mut vec = Vec::new();
                    while let Some(TermOrPush::Term(term, repeat)) =
                        stack.pop() {
                        vec.push((term, repeat.unwrap_or(1)));
                    }
                    stack.push(TermOrPush::Term(
                        Term::Compound(vec),
                        None));
                },
                _ => unreachable!(),
            }
        }

        let Some(&TermOrPush::Term(ref term, None)) = stack.first() else {
            unreachable!()
        };

        let freq = {
            let mut freq = HashMap::<AtomName, u32>::new();
            let mut stack = Vec::new();
            stack.push((term, 1));
            while let Some((term, repeat)) = stack.pop() {
                match *term {
                    Term::Atom(ref term) =>
                        freq.entry(term.clone())
                            .or_default()
                            .add_assign(repeat),
                    Term::Compound(ref segs) =>
                        stack.extend(
                            segs.iter()
                                .map(|&(ref s, n)| (s, n * repeat)))
                }
            }
            freq
        };

        let mut freq = freq
            .into_iter()
            .collect::<Vec<_>>();
        freq.sort_unstable();

        let mut s = String::new();
        for &(ref atom, n) in &freq {
            let _ = write!(&mut s, "{atom}").err();
            if n >= 2 {
                let _ = write!(&mut s, "{n}").err();
            }
        }

        s
    }
}

fn main() {
    assert_eq!(solution::main("H2O".into()), "H2O");
    assert_eq!(solution::main("Mg(OH)2".into()), "H2MgO2");
    assert_eq!(solution::main("K4(ON(SO3)2)2".into()), "K4N2O14S4");
    assert_eq!(solution::main("Hdadaf2OKadlkfadkaldfa2H".into()), "HHdadaf2Kadlkfadkaldfa2O");
}
