use std::{fs::File, io::{BufRead, BufReader}, ops::RangeInclusive};

const ROWCOL: usize = 135;

fn main() {
    // Main file 135 x 135
    let file: File = File::open("src\\day_4\\input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut shelfstate: [[bool; ROWCOL]; ROWCOL] = [[false; _]; _];

    for (row, line) in reader.lines().enumerate() {
        let line = line.expect("Line read failed fuck you");

        for (idx, paper) in line.char_indices() {
            match paper {
                '.' => { // no toilet
                    shelfstate[row][idx] = false;
                },
                '@' => { // toilet
                    shelfstate[row][idx] = true;
                },
                _ => unreachable!()
            }
        }
    }

    let res1 = solve(&mut shelfstate, false);
    let res2 = solve(&mut shelfstate, true);

    println!("RES 1: {}", res1);
    println!("RES 2: {}", res2);
}

fn solve(state: &mut [[bool; ROWCOL]; ROWCOL], take_max: bool) -> u32 {
    let mut got_one: bool = true;
    let mut res: u32 = 0;

    while got_one {
        got_one = false;

        let mut state_to_update_everitim: [[bool; ROWCOL]; ROWCOL] = state.clone();

        let mut top_paper: [bool; 3] = [false; _];
        let mut bot_paper: [bool; 3] = [false; _];
        let mut sid_paper: [bool; 2] = [false; _];

        for (row_idx, row) in state.iter().enumerate() {

            for (col_idx, item) in row.iter().enumerate() {

                let row_lower_bound: usize = col_idx + (col_idx > 0) as usize;
                let row_upper_bound: usize = (col_idx + 1) - (col_idx == ROWCOL - 1) as usize;
                let row_range = row_lower_bound..=row_upper_bound;

                // Getting rolls in adjacent rows
                if row_idx == 0 {
                    get_row_window(state, row_idx + 1, col_idx, row_range, &mut top_paper);
                } else if row_idx == ROWCOL - 1 {
                    get_row_window(state, row_idx - 1, col_idx, row_range, &mut bot_paper);
                } else {
                    get_row_window(state, row_idx - 1, col_idx, row_range.clone(), &mut bot_paper);
                    get_row_window(state, row_idx + 1, col_idx, row_range        , &mut top_paper);
                }

                // Getting rolls to the left n right
                if col_idx == 0 {
                    sid_paper[0] = row[1];
                } else if col_idx == (ROWCOL - 1) {
                    sid_paper[1] = false;
                } else {
                    sid_paper[1] = row[col_idx + 1]
                }

                // Summation cuz we've found bot, top and sides
                let adjacent_amount: u8 = {
                    top_paper.iter()
                        .map(|b| *b as u8)
                        .sum::<u8>() +

                    bot_paper.iter()
                        .map(|b| *b as u8)
                        .sum::<u8>() +

                    sid_paper.iter()
                        .map(|b| *b as u8)
                        .sum::<u8>()
                };

                // found dat paper
                if adjacent_amount < 4 && *item {
                    got_one = true;
                    state_to_update_everitim[row_idx][col_idx] = false;
                    res += 1;
                }

                // Insert the curr item into the buffer at the end so that the next iteration has access to it
                sid_paper[0] = *item;
            }

            // Clearing the buhf
            top_paper.fill(false);
            bot_paper.fill(false);
        }

        if !take_max { break }

        // Setting the state to the version that has been updated throughout the iteration :D
        *state = state_to_update_everitim;
    }
    res
}

#[inline]
fn get_row_window(state: &[[bool; ROWCOL]; ROWCOL], row_idx: usize, col_idx: usize, range: RangeInclusive<usize>, curr_paper_buf: &mut [bool; 3]) {
    for idx in range {
        // Modulo here cuz we do not need to care abt what order the rolls are in, only if there are any rolls or not
        // Mod also used to not index out of bounds, and instead wrap around to 0 index when idx is a multiple of 3
        curr_paper_buf[idx % 3] = state[row_idx][idx];
    }

    // This is to remove the residual roll (if there is a roll in that pos) of "(col_idx + 1) % 3" because
    // the buffer is of size 3, and as stated above it sorta rotates around due to the modulo operation, and if I simulate the
    // rotation of rolls in a row of 10, the buffer would be altered like this (where '->' is a new iteration):
    // [0. 1, 0] -> [0, 1, 2] -> [3, 1, 2] -> [3, 4, 2] -> [3, 4, 5] -> [6, 4, 5] -> [6, 7, 5] -> [6, 7, 8] -> [9, 7, 8] ( -> [9, 0, 8] )
    //
    // The problem is now that since at the boundaries of the shelf of rolls, we dont want to let the 7th possible roll have an
    // effect on the amount of neighbouring rolls, so the 7th roll below is always set to false if iteration is complete:
    if col_idx == ROWCOL - 1 {
        curr_paper_buf[(col_idx + 1) % 3] = false;
    }
}
