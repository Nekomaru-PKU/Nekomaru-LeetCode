fn solution(num: i32) -> String {
    if num == 0 { return "Zero".into() }

    const WORDS: [&[u8]; 20] = [
        b"",
        b"One ",
        b"Two ",
        b"Three ",
        b"Four ",
        b"Five ",
        b"Six ",
        b"Seven ",
        b"Eight ",
        b"Nine ",
        b"Ten ",
        b"Eleven ",
        b"Twelve ",
        b"Thirteen ",
        b"Fourteen ",
        b"Fifteen ",
        b"Sixteen ",
        b"Seventeen ",
        b"Eighteen ",
        b"Nineteen ",
    ];

    const WORDS_TENS: [&[u8]; 10] = [
        b"",
        b"",
        b"Twenty ",
        b"Thirty ",
        b"Forty ",
        b"Fifty ",
        b"Sixty ",
        b"Seventy ",
        b"Eighty ",
        b"Ninety ",
    ];

    const WORDS_HUNDRED : &[u8] = b"Hundred ";
    const WORDS_THOUSAND: &[u8] = b"Thousand ";
    const WORDS_MILLION : &[u8] = b"Million ";
    const WORDS_BILLION : &[u8] = b"Billion ";

    fn convert_in_thousand(n: usize, s: &mut Vec<u8>) {
        debug_assert!(n <= 1_000);
        if n >= 100 {
            s.extend_from_slice(WORDS[n / 100]);
            s.extend_from_slice(WORDS_HUNDRED);
        }

        let n = n % 100;
        if n >= 20 {
            s.extend_from_slice(WORDS_TENS[n / 10]);
            s.extend_from_slice(WORDS[n % 10]);
        } else {
            s.extend_from_slice(WORDS[n]);
        }
    }

    // the longest word sequence would be
    // "777 Million 777 Thousand 777".len() == 31 * 3 + 19 = 112, since
    // "Seventeen Hundred Seventy Seven".len() == 31.
    let mut s = Vec::with_capacity(128);
    let mut n = num as usize;

    if n >= 1_000_000_000 {
        convert_in_thousand(n / 1_000_000_000, &mut s);
        s.extend_from_slice(WORDS_BILLION);
        n %= 1_000_000_000;
    }

    if n >= 1_000_000 {
        convert_in_thousand(n / 1_000_000, &mut s);
        s.extend_from_slice(WORDS_MILLION);
        n %= 1_000_000;
    }

    if n >= 1_000 {
        convert_in_thousand(n / 1_000, &mut s);
        s.extend_from_slice(WORDS_THOUSAND);
        n %= 1_000;
    }

    convert_in_thousand(n % 1_000, &mut s);
    s.pop(); // remove the ending space here!
    s.shrink_to_fit();
    String::from_utf8(s).unwrap()
}

fn main() {
    assert_eq!(solution(100), "One Hundred");
    assert_eq!(solution(123), "One Hundred Twenty Three");
    assert_eq!(solution(12345), concat!(
        "Twelve Thousand ",
        "Three Hundred Forty Five"));
    assert_eq!(solution(1_234_567), concat!(
        "One Million ",
        "Two Hundred Thirty Four Thousand ",
        "Five Hundred Sixty Seven"));
    assert_eq!(solution(i32::MAX), concat!(
        "Two Billion ",
        "One Hundred Forty Seven Million ",
        "Four Hundred Eighty Three Thousand ",
        "Six Hundred Forty Seven"));
}
