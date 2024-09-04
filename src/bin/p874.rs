fn solution(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;
    let obstacles = obstacles
        .iter()
        .map(|vec| (vec[0], vec[1]))
        .collect::<HashSet<_>>();
    commands
        .iter()
        .scan(
            ((0, 0), (0, 1)),
            |&mut (ref mut pos, ref mut dir), &command| {
                #[cfg(debug_assertions)]
                println!("pos: {pos:?}, dir: {dir:?}");
                match command {
                    -2 => *dir = (-dir.1, dir.0),
                    -1 => *dir = (dir.1, -dir.0),
                    1..=9 => {
                        for _ in 0..command {
                            let next_pos = (
                                pos.0 + dir.0,
                                pos.1 + dir.1);
                            if obstacles.contains(&next_pos) {
                                break;
                            }
                            *pos = next_pos;
                        }
                    },
                    _ => panic!("unexpected command"),
                }
                Some(pos.0 * pos.0 + pos.1 * pos.1)
            })
        .max()
        .unwrap_or_default()
}

fn main() {
    assert_eq!(solution(vec![], vec![]), 0);
    assert_eq!(solution(vec![4, -1, 3], vec![]), 25);
    assert_eq!(solution(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]), 65);
    assert_eq!(solution(vec![6, -1, -1, 6], vec![]), 36);
}
