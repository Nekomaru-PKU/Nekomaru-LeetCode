fn solution(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    // <time: O(shelf_width * n), space: O(shelf_width)>

    use std::mem;

    // let `dp[i][j].0` be the minimum possible height (the last shelf
    // not included) after placing book `0..i` and the last shelf has
    // `j` width remaining.
    // 
    // let `dp[i][j].1` be the height of the last shelf after placing
    // book `0..i` and the last shelf has `j` width remaining.
    // note that this value can be uniquely determined by pair `(i, j)`
    // and is stored only as a cache (rather than a part of the dp memo).
    // 
    // we use a swap buffer to store `dp[i][j]` to save space.
    let mut dp = vec![None; shelf_width as _];
    let mut dp_next = dp.clone();

    // here we place the first book:
    dp[(shelf_width - books[0][0]) as usize] = Some((0, books[0][1]));
    #[cfg(debug_assertions)] println!("{:?}", &dp);

    // then the rest books:
    for book in &books[1..] {
        let book_thickness = book[0];
        let book_height = book[1];

        // put `book` on a new shelf:
        dp_next[(shelf_width - book_thickness) as usize] = Some((
            dp  .iter()
                .flatten()
                .map(|&(h0, h1)| h0 + h1)
                .min()
                .unwrap(),
            book_height));

        // put `book` on the last shelf if it has enough space for it:
        for remaining_space in book_thickness..shelf_width {
            if let Some((h0, h1)) = dp[remaining_space as usize] {
                dp_next[(remaining_space - book_thickness) as usize] = Some((
                    h0,
                    h1.max(book_height)));
            }
        }

        mem::swap(&mut dp, &mut dp_next);
        dp_next.fill(None);

        #[cfg(debug_assertions)] println!("{:?}", &dp);
    }

    dp.iter()
        .flatten()
        .map(|&(h0, h1)| h0 + h1)
        .min()
        .unwrap()
}

fn main() {
    use leetcode::input::Input;
    assert_eq!(solution([[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]].input(), 4), 6);
    assert_eq!(solution([[1, 3], [2, 4], [3, 2]].input(), 6), 4);
}
