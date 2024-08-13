mod solution {
    pub fn main(
        rows: i32,
        cols: i32,
        r_start: i32,
        c_start: i32)
     -> Vec<Vec<i32>> {
        let num_cells = (rows * cols) as usize;

        let mut steps = Vec::with_capacity(num_cells);
        let mut last_step = (i32::MAX, i32::MAX);
        let mut push_step = |row, col| -> bool {
            if  last_step != (row, col) {
                steps.push(vec![row, col]);
                last_step = (row, col);
            }
            steps.len() >= num_cells
        };

        push_step(r_start, c_start);

        enum Direction { East, West, South, North }
        let mut direction = Direction::East;
        let mut edge = 0;
        let mut r_turn = r_start;
        let mut c_turn = c_start;

        'outer: loop {
            if let Direction::East | Direction::West = direction {
                edge += 1;
            }

            let (r_turn_old, c_turn_old) = (r_turn, c_turn);
            let (r_turn_new, c_turn_new) = match direction {
                Direction::East  => (r_turn_old, c_turn_old + edge),
                Direction::West  => (r_turn_old, c_turn_old - edge),
                Direction::South => (r_turn_old + edge, c_turn_old),
                Direction::North => (r_turn_old - edge, c_turn_old),
            };

            match direction {
                Direction::East | Direction::West =>
                    if r_turn >= 0 && r_turn < rows {
                        let c_head = c_turn_old.clamp(0, cols - 1);
                        let c_tail = c_turn_new.clamp(0, cols - 1);
                        if c_head <= c_tail {
                            for c in c_head..=c_tail {
                                if push_step(r_turn, c) {
                                    break 'outer;
                                }
                            }
                        } else {
                            for c in (c_tail..=c_head).rev() {
                                if push_step(r_turn, c) {
                                    break 'outer;
                                }
                            }
                        }
                    },
                Direction::South | Direction::North =>
                    if c_turn >= 0 && c_turn < cols {
                        let r_head = r_turn_old.clamp(0, rows - 1);
                        let r_tail = r_turn_new.clamp(0, rows - 1);
                        if r_head <= r_tail {
                            for r in r_head..=r_tail {
                                if push_step(r, c_turn) {
                                    break 'outer;
                                }
                            }
                        } else {
                            for r in (r_tail..=r_head).rev() {
                                if push_step(r, c_turn) {
                                    break 'outer;
                                }
                            }
                        }
                    },
            }

            r_turn = r_turn_new;
            c_turn = c_turn_new;
            direction = match direction {
                Direction::East  => Direction::South,
                Direction::South => Direction::West,
                Direction::West  => Direction::North,
                Direction::North => Direction::East,
            }
        }

        steps
    }
}

fn main() {
    let _ = solution::main(1, 1, 0, 0);
}
