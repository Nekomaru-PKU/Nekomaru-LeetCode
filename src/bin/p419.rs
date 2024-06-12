mod solution {
    pub fn main(board: Vec<Vec<char>>) -> i32 {
        #[allow(path_statements)] {
            self::main_3pass;
            self::main_2pass(board)
        }
    }

    pub fn main_3pass(mut board: Vec<Vec<char>>) -> i32 {
        /// `O` for empty, `X` for battle ship.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum History { O, OX, XX }

        let rows = board.len();
        let columns = board[0].len();

        let mut total = 0;

        for row in &mut board {
            let mut history = History::O;
            for j in 0..columns {
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
                    _ => unreachable!(),
                }
            }
        }

        for j in 0..columns {
            let mut history = History::O;
            for i in 0..rows {
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
                    _ => unreachable!(),
                }
            }
        }

        total += board.iter().flatten().filter(|&&c| c == 'X').count() as i32;
        total
    }

    pub fn main_2pass(mut board: Vec<Vec<char>>) -> i32 {
        let rows = board.len();
        let columns = board[0].len();

        let mut count = 0;
        for column in 0..columns {
            let mut prev_parts: u32 = 0;
            for row in 0..rows {
                match board[row][column] {
                    'X' => {
                        match prev_parts {
                            0 => (),
                            1 => {
                                count += 1;
                                board[row - 1][column] = '.';
                                board[row][column] = '.';
                            },
                            2.. => {
                                board[row][column] = '.';
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
}

fn main() {
    assert_eq!(solution::main(vec![vec!['.']]), 0);
    assert_eq!(solution::main(vec![
        vec!['X','.','.','X'],
        vec!['.','.','.','X'],
        vec!['.','.','.','X']]
    ), 2);
    assert_eq!(solution::main(vec![
        vec!['X','.','.','X'],
        vec!['.','.','.','X'],
        vec!['X','X','.','X']]
    ), 3);
}
