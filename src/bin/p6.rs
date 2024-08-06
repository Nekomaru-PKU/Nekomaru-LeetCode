fn solution(s: String, num_rows: i32) -> String {
    if num_rows == 1 { return s; }

    let s = s.into_bytes();

    let num_rows = num_rows as usize;
    let group_size = num_rows * 2 - 2;
    let num_groups = (s.len() + group_size - 1) / group_size;

    let mut vec = Vec::with_capacity(s.len());

    /* for row = 0 */ {
        let mut group_begin = 0;
        for _ in 0..num_groups {
            if let Some(&c) = s.get(group_begin) {
                vec.push(c);
            }
            group_begin += group_size;
        }
    }

    for row in 1..num_rows - 1 {
        let mut group_begin = 0;
        for _ in 0..num_groups {
            if let Some(&c) = s.get(group_begin + row) {
                vec.push(c);
            }
            if let Some(&c) = s.get(group_begin + group_size - row) {
                vec.push(c);
            }
            group_begin += group_size;
        }
    }

    /* for row = num_rows - 1 */ {
        let mut group_begin = 0;
        for _ in 0..num_groups {
            if let Some(&c) = s.get(group_begin + num_rows - 1) {
                vec.push(c);
            }
            group_begin += group_size;
        }
    }

    String::from_utf8(vec).unwrap()
}

fn main() {
    assert_eq!(solution("PAYPALISHIRING".into(), 3), "PAHNAPLSIIGYIR");
    assert_eq!(solution("PAYPALISHIRING".into(), 4), "PINALSIGYAHRPI");
    assert_eq!(solution("A".into(), 1), "A");
}
