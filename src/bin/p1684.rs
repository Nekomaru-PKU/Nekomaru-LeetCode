fn solution(allowed: String, words: Vec<String>) -> i32 {
    let allowed = allowed
        .into_bytes()
        .into_iter()
        .fold([false; 26], |mut acc, c| {
            acc[(c - b'a') as usize] = true;
            acc
        });
    words
        .into_iter()
        .map(String::into_bytes)
        .filter(|word| {
            word.iter()
                .all(|c| allowed[(c - b'a') as usize])
        })
        .count() as _
}
 
fn main() {
    use leetcode::input::Input;
    assert_eq!(solution("ab".into(), ["ad", "bd" ,"aaab", "baa", "badab"].input()), 2);
    assert_eq!(solution("abc".into(), ["a", "b", "c", "ab", "ac", "bc", "abc"].input()), 7);
    assert_eq!(solution("cad".into(), ["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"].input()), 4);
}
