mod solution {
    use std::{
        cmp::Ordering,
        collections::HashSet,
    };

    #[derive(Debug, Clone, Eq)]
    struct DigitArray {
        arr: [u8; 24],
        len: usize,
    }

    impl PartialEq for DigitArray {
        fn eq(&self, other: &Self) -> bool {
            self.arr[..self.len].eq(
                &other.arr[..other.len])
        }
    }

    impl DigitArray {
        const fn from_u64(mut num: u64) -> Self {
            let mut arr = [0; 24];
            let mut len = 0;
            while num != 0 {
                arr[len] = (num % 10) as _;
                num /= 10;
                len += 1;
            }
            Self { arr, len }
        }

        fn to_u64(&self) -> u64 {
            let mut num = 0;
            for &digit in &self.arr[0..self.len] {
                num *= 10;
                num += digit as u64;
            }
            num
        }

        fn rev(&self) -> Self {
            let Self { mut arr, len } = self.clone();
            for i in 0..(len / 2) {
                arr.swap(i, len - i - 1);
            }
            Self { arr, len }
        }

        fn is_palindrome(&self) -> bool {
            self.rev().eq(self)
        }

        fn mul_pow_of_ten(&mut self, exp: usize) {
            for i in (0..self.len).rev() {
                self.arr[i + exp] = self.arr[i];
            }
            self.len += exp;
        }

        fn div_pow_of_ten(&mut self, exp: usize) {
            for i in exp..self.len {
                self.arr[i - exp] = self.arr[i];
            }
            self.len -= exp;
            
        }

        fn add_one(&mut self) -> bool {
            let mut carry = 1;
            let mut i = 0;
            while carry > 0 && i < self.len {
                let sum = self.arr[i] + carry;
                self.arr[i] = sum % 10;
                carry = sum / 10;
                i += 1;
            }
            carry > 0
        }

        fn sub_one(&mut self) -> bool {
            for i in 0..self.len {
                if self.arr[i] > 0 {
                    self.arr[i] -= 1;
                    return false;
                }
                self.arr[i] = 9;
            }
            self.arr[self.len - 1] == 0
        }
    }

    fn update_result_with_palindrome(
        num: u64,
        pal: u64,
        out: &mut u64) {
        if pal == num { return; }
        match u64::cmp(
            &num.abs_diff(pal),
            &num.abs_diff(*out)) {
            Ordering::Less  => *out = pal,
            Ordering::Equal => *out = (*out).min(pal),
            Ordering::Greater => (),
        }
    }

    pub fn core_naive(num: u64) -> u64 {
        if num == 0 { return 1; }
        let mut out = 0u64;
        for i in 0..num * 10 {
            if  i != num &&
                i.abs_diff(num) < out.abs_diff(num) &&
                DigitArray::from_u64(i).is_palindrome() {
                out = i;
            }
        }
        out
    }

    struct PalindromeCache {
        data: HashSet<u64>,
        max_digits: u32,
    }

    impl PalindromeCache {
        fn new() -> Self {
            Self {
                data: (0..=9)
                    .flat_map(|n| [n, n * 11])
                    .collect(),
                max_digits: 2,
            }
        }

        fn iter(&self) -> impl Iterator<Item = u64> + '_ {
            self.data.iter().copied()
        }

        fn ensure(&mut self, num: u64) {
            while num >= 10u64.pow(self.max_digits) {
                self.max_digits += 1;
                let half_n = (self.max_digits + 1) / 2;
                for num in 10u64.pow(half_n - 1)..10u64.pow(half_n) {
                    let mut num = DigitArray::from_u64(num);
                    num.mul_pow_of_ten((self.max_digits - half_n) as _);
                    for i in 0..(num.len / 2) {
                        num.arr[i] = num.arr[num.len - i - 1];
                    }
                    self.data.insert(num.to_u64());
                }

                #[cfg(debug_assertions)]
                println!("ensure max_digits: {} => {:?}", self.max_digits, {
                    let mut vec = self.data.iter().collect::<Vec<_>>();
                    vec.sort_unstable();
                    vec
                });
            }
        }
    }

    pub fn core_cached(num: u64) -> u64 {
        if num == 0 { return 1; }

        use std::cell::RefCell;
        thread_local! {
            static CACHE: RefCell<PalindromeCache> =
                RefCell::new(PalindromeCache::new());
        }

        let mut out = 0u64;
        CACHE.with_borrow_mut(|cache| {
            cache.ensure(num * 10);
            for pal in cache.iter() {
                update_result_with_palindrome(num, pal, &mut out);
            }
        });
        out
    }

    pub fn main_opt(num: String) -> String {
        let num = num.parse::<u64>().unwrap();
        core_opt(num).to_string()
    }

    pub fn core_opt(num: u64) -> u64 {
        if num < 100 {
            return match num {
                0 => 1,
                1..=10 => num - 1,
                11 => 9,
                22|33|44|55|66|77|88 => num - 11,
                12..=16 => 11,
                17..=27 => 22,
                28..=38 => 33,
                39..=49 => 44,
                50..=60 => 55,
                61..=71 => 66,
                72..=82 => 77,
                83..=93 => 88,
                94..=98 => 99,
                99 => 101,
                100.. => unreachable!(),
            };
        }

        let mut out = 0;
        for pal in (num.max(10) - 10)..=(num + 10) {
            if  pal != num &&
                num.abs_diff(pal) < num.abs_diff(out) &&
                DigitArray::from_u64(pal).is_palindrome() {
                out = pal;
            }
        }

        fn mirror_high_to_low(mut num: DigitArray, even: bool) -> DigitArray {
            num.mul_pow_of_ten(if even { num.len } else { num.len - 1 });
            for i in 0..(num.len / 2) {
                num.arr[i] = num.arr[num.len - i - 1];
            }
            num
        }

        let mut digits = DigitArray::from_u64(num);
        let even = digits.len % 2 == 0;
        digits.div_pow_of_ten(digits.len / 2);
        update_result_with_palindrome(
            num,
            mirror_high_to_low(digits.clone(), even).to_u64(),
            &mut out);

        let mut digits_new = digits.clone();
        if !digits_new.add_one() {
            update_result_with_palindrome(
                num,
                mirror_high_to_low(digits_new, even).to_u64(),
                &mut out);
        }

        let mut digits_new = digits.clone();
        if !digits_new.sub_one() {
            update_result_with_palindrome(
                num,
                mirror_high_to_low(digits_new, even).to_u64(),
                &mut out);
        }

        out
    }
}

fn main() {
    assert_eq!(solution::core_naive(0), 1);
    assert_eq!(solution::core_naive(1), 0);
    assert_eq!(solution::core_naive(123), 121);
    assert_eq!(solution::core_naive(197), 202);
    assert_eq!(solution::core_naive(1000), 999);
    assert_eq!(solution::core_naive(1999), 2002);

    assert_eq!(solution::main_opt("0".into()), "1");
    assert_eq!(solution::main_opt("1".into()), "0");
    assert_eq!(solution::main_opt("123".into()), "121");

    println!();
    println!("testing solution::core_cached...");
    compare_solution(
        solution::core_cached,
        solution::core_naive,
        12_222);
    println!();
    println!("testing solution::core_opt...");
    compare_solution(
        solution::core_opt,
        solution::core_cached,
        222_222);
}

fn compare_solution(
    fn0: fn(u64) -> u64,
    fn1: fn(u64) -> u64,
    max: u64) {
    use std::time::Instant;

    assert_eq!(fn0(0), fn1(0), "mismatch at n = 0");

    let mut prev = Instant::now();
    for num in 1..=max {
        assert_eq!(fn0(num), fn1(num), "mismatch at num = {num}");

        if num % 1000 == 0 {
            let now = Instant::now();
            let duration_ms = now
                .duration_since(prev)
                .as_millis();
            prev = now;
            println!(
                "{}..={} checked in {}.{}s",
                num - 1000,
                num,
                duration_ms / 1000,
                duration_ms % 1000);
        }
    }
}
