fn solution(items: Vec<Vec<i32>>, k: i32) -> i64 {
    use std::collections::HashMap;
    use std::iter;
    let k = k as usize;

    // we reshape this problem as if we have some categories, and for each category `i`
    // we have some items, each of which have a total profit of `categories[i][j]`
    let mut categories = HashMap::<_, Vec<_>>::with_capacity(10000);
    for item in &items {
        let (profit, category) = (item[0], item[1]);
        categories.entry(category).or_default().push(profit);
    }

    // after the reshape we do not care about the category indices anymore,
    // just the profit of items in each category matters.
    let mut categories = categories.into_values().collect::<Vec<_>>();

    // we sort categories[i] for each i in descending order, since it is always better
    // to pick items of higher profit first for a given category.
    for items in &mut categories {
        items.sort_unstable_by_key(|n| -n);
    }

    // we define dp[m][n] as the *total benefit* we can get at maximum elegance by selecting `n` items
    // from category range 0..m where m <= category.len(), n <= k.
    // immediately we have:
    //      dp[m][n] = max_by_elegance {
    //          // if we pick nothing from the current category
    //          total_benefit: dp[m - 1][n], elegance: +,
    //          // we pick some items from the current category to replace the best
    //          // choice without the current category, and get the extra distinction
    //          // bonus of m^2 - (m - 1)^2 = 2*m - 1
    //          dp[m - 1][n - d] + categories[m][0..d].sum() + 2*m - 1
    //              for each d in 0..min(n, categories[i].len())
    //      }
    let mut dp: Vec<Vec<i64>> = iter::repeat(
        iter::repeat(0)
            .take(k + 1)
            .collect())
        .take(categories.len() + 1)
        .collect();
    for m in 1..=categories.len() {
        // the current iteration adds category m - 1
        let items = &categories[m - 1];
        for n in 1..=k {
            let mut max = dp[m - 1][n];
            for d in 1..=n.min(items.len()) {
                max = max.max(
                    dp[m - 1][n - d] +
                    items[0..d].iter().map(|n| *n as i64).sum::<i64>() +
                    2 * (m as i64) - 1);
            }
            dp[m][n] = max;
        }
    };

    dp[categories.len()][k]
}

fn main() {
    use leetcode::input::Input;

    // simple cases
    assert_eq!(solution([[1, 1]].input(), 1), 2);
    assert_eq!(solution([[1, 1], [1, 1]].input(), 2), 3);

    // provided cases
    assert_eq!(solution([[3, 2], [5, 1], [10, 1]].input(), 2), 17);
    assert_eq!(solution([[3, 1], [3, 1], [2, 2], [5, 3]].input(), 3), 19);
    assert_eq!(solution([[1, 1], [2, 1], [3, 1]].input(), 3), 7);
}
