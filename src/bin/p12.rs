fn solution(mut num: i32) -> String {
    let mut out = Vec::with_capacity(31);
    while num >= 1000 {
        out.push(b'M');
        num -= 1000;
    }

    fn convert_one_digit(
        num: &mut i32,
        out: &mut Vec<u8>,
        base: i32,
        symbol_1x: u8,
        symbol_5x: u8,
        symbol_10x: u8) {
        if *num >= 9 * base {
            out.push(symbol_1x);
            out.push(symbol_10x);
            *num -= 9 * base;
        }

        if *num >= 5 * base {
            out.push(symbol_5x);
            *num -= 5 * base;
        }

        if *num >= 4 * base {
            out.push(symbol_1x);
            out.push(symbol_5x);
            *num -= 4 * base;
        }

        if *num >= base {
            for _ in 0..*num / base {
                out.push(symbol_1x);
            }
            *num %= base;
        }
    }

    convert_one_digit(
        &mut num,
        &mut out,
        100,
        b'C', b'D', b'M');
    convert_one_digit(
        &mut num,
        &mut out,
        10,
        b'X', b'L', b'C');
    convert_one_digit(
        &mut num,
        &mut out,
        1,
        b'I', b'V', b'X');

    String::from_utf8(out).unwrap()
}

fn main() {
    assert_eq!(solution(3749), "MMMDCCXLIX");
    assert_eq!(solution(58), "LVIII");
    assert_eq!(solution(1994), "MCMXCIV");
}
