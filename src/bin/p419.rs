mod solution {
    pub fn main(mut board: Vec<Vec<char>>) -> i32 {
        /// `O` for empty, `X` for battle ship.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum History { O, OX, XX }

        let rows = board.len();
        let columns = board[0].len();

        let mut total = 0;

        for i in 0..rows {
            let mut history = History::O;
            for j in 0..columns {
                match board[i][j] {
                    '.' => history = History::O,
                    'X' => match history {
                        History::O => {
                            history = History::OX;
                        },
                        History::OX => {
                            history = History::XX;
                            total += 1;
                            board[i][j - 1] = '.';
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
