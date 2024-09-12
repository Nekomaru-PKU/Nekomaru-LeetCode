#![expect(clippy::match_on_vec_items)]

fn solution_3pass(mut board: Vec<Vec<char>>) -> i32 {
    /// `O` for empty, `X` for battle ship.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum History { O, OX, XX }

    let num_rows = board.len();
    let num_cols = board[0].len();

    let mut total = 0;

    for row in &mut board {
        let mut history = History::O;
        for j in 0..num_cols {
            match row[j] {
                '.' => history = History::O,
                'X' => match history {
                    History::O => {
                        history = History::OX;
                    },
                    History::OX => {
                        history = History::XX;
                        total += 1;
                        row[j - 1] = '.';
                        row[j] = '.';
                    },
                    History::XX => {
                        row[j] = '.';
                    },
                },
                _ => panic!(),
            }
        }
    }

    for j in 0..num_cols {
        let mut history = History::O;
        for i in 0..num_rows {
            match board[i][j] {
                '.' => history = History::O,
                'X' => match history {
                    History::O => {
                        history = History::OX;
                    },
                    History::OX => {
                        history = History::XX;
                        total += 1;
                        board[i - 1][j] = '.';
                        board[i][j] = '.';
                    },
                    History::XX => {
                        board[i][j] = '.';
                    },
                },
                _ => panic!(),
            }
        }
    }

    total += board.iter().flatten().filter(|&&c| c == 'X').count() as i32;
    total
}

fn solution_2pass(mut board: Vec<Vec<char>>) -> i32 {
    let num_rows = board.len();
    let num_cols = board[0].len();

    let mut count = 0;
    for col in 0..num_cols {
        let mut prev_parts: u32 = 0;
        for row in 0..num_rows {
            match board[row][col] {
                'X' => {
                    match prev_parts {
                        0 => (),
                        1 => {
                            count += 1;
                            board[row - 1][col] = '.';
                            board[row][col] = '.';
                        },
                        2.. => {
                            board[row][col] = '.';
                        },
                    }
                    prev_parts += 1;
                },
                _ => prev_parts = 0,
            }
        }
    }

    for row in &board {
        let mut prev_empty = true;
        for &c in row {
            if prev_empty && c == 'X' {
                count += 1;
            }
            prev_empty = c != 'X';
        }
    }

    count
}

fn main() {
    for solution in [
        solution_2pass,
        solution_3pass
    ] {
        assert_eq!(solution(vec![vec!['.']]), 0);
        assert_eq!(solution(vec![
            vec!['X','.','.','X'],
            vec!['.','.','.','X'],
            vec!['.','.','.','X']]
        ), 2);
        assert_eq!(solution(vec![
            vec!['X','.','.','X'],
            vec!['.','.','.','X'],
            vec!['X','X','.','X']]
        ), 3);
    }
}
