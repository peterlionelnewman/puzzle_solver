use colored::Colorize;
use itertools::iproduct;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::iter::Iterator;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::vec::Vec;

/*

   Puzzle Solver, by Pete in Rust~!
   ####################
   This project is about learning Rust / my first Rust project. It solcves the
   puzzle in the resources folder of this project

   STRATEGY:
   bogo place the randomly flipped and rotated pieces along the left most side or top row:

       1. pick a random piece
       2. rotate the board
       3. rotate the piece
       4. randomly flip the piece
       5. pick a random location along the left or top empty edge
       6. fit the piece

*/

fn initiate_board() -> ([[u16; 7]; 7], Vec<Vec<Vec<u16>>>) {
    /*

    make the board - as vec of vec array 7x7
    values == 256: 'wall'
    1 > values > 255: 'pieces'
    values == 0 are: 'unsolved'

    setting space values to indicate 'out of bounds'
    piece value = 0, 1, 2, 4, 8, 16, 32, 64, 128, 256,
    piece index = 0  1, 2, 3, 4, 5,  6,  7,  8,   'out of bounds'

    values increase such that sum of (piece values < current index) < piece value

     */

    let starting_board: [[u16; 7]; 7] = [
        [0, 0, 0, 0, 0, 0, 256],
        [0, 0, 256, 0, 0, 0, 256],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 256, 0, 256, 256, 256, 256],
    ];

    // let mut empty_spaces_in_attempt: u16 = sum_blank_spaces(starting_board.clone());

    // starting_board[1][2] = 256; // September
    // starting_board[6][1] = 256; // 30th

    // starting_board[1][3] = 256; // October
    // starting_board[3][4] = 256; // 12th

    // make the 8 puzzle pieces

    // create array of vec of vec containing the pieces
    let pieces = vec![
        vec![
            // 0
            vec![1, 1, 1, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            // 1
            vec![0, 2, 2, 0],
            vec![0, 2, 0, 0],
            vec![2, 2, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            // 2
            vec![0, 4, 4, 4],
            vec![4, 4, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            // 3
            vec![0, 0, 0, 8],
            vec![8, 8, 8, 8],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            // 4
            vec![0, 16, 0, 0],
            vec![16, 16, 16, 16],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            // 5
            vec![0, 32, 32, 0],
            vec![32, 32, 32, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            // 6
            vec![64, 64, 64, 0],
            vec![64, 0, 64, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            // 7
            vec![128, 128, 128, 0],
            vec![128, 128, 128, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
    ]
    .to_vec();

    (starting_board, pieces)
}

fn board_image(b: [[u16; 7]; 7]) {
    let mut s = String::new();
    for b_rows in b.iter() {
        for b_cols in b_rows.iter() {
            if *b_cols == 0 {
                s += &format!("\x1B[31m{:03}\x1B[0m ", b_cols);
            } else {
                s += &format!("{:03} ", b_cols);
            }
        }
        s += "\n";
    }
    s += "\n";
    print!("{}", s);
}

#[inline]
fn calc_perimeter(b: [[u16; 7]; 7]) -> u16 {
    let mut perimeter = 0;
    let b_padded = [[1; 9]; 9];
    for i in 1..8 {
        for j in 1..8 {
            match b[i - 1][j - 1] {
                0 => {
                    if i > 0 && b_padded[i][j - 1] != 0 {
                        perimeter += 1;
                    }
                    if i < 7 && b_padded[i][j + 1] != 0 {
                        perimeter += 1;
                    }
                    if j > 0 && b_padded[i - 1][j] != 0 {
                        perimeter += 1;
                    }
                    if j < 7 && b_padded[i + 1][j] != 0 {
                        perimeter += 1;
                    }
                }
                _ => (),
            }
        }
    }
    perimeter
}

#[inline]
fn sum_blank_spaces(b: [[u16; 7]; 7]) -> u16 {
    let mut sum = 0;
    // for row in b.iter() {
    //     for &e in row.iter() {
    //         if e == 0 {
    //             sum += 1;
    //         }
    //     }
    // }
    for i in 0..7 {
        for j in 0..7 {
            if b[i][j] == 0 {
                sum += 1;
            }
        }
    }

    sum
}

#[inline]
fn valid_board(b: [[u16; 7]; 7]) -> bool {
    let mut valid = true;
    for row in b.iter() {
        for &e in row.iter() {
            if e > 256 {
                valid = false;
                break;
            }
        }
        if !valid {
            break;
        }
    }
    valid
}

#[inline]
fn mirror_piece(v: Vec<Vec<u16>>) -> Vec<Vec<u16>> {
    let mut m = v.clone();
    for row in m.iter_mut() {
        row.reverse();
    }
    m
}

#[inline]
fn trans_piece(v: Vec<Vec<u16>>) -> Vec<Vec<u16>> {
    let len = v[0].len();
    let mut transposed = Vec::with_capacity(len);
    for _ in 0..len {
        transposed.push(Vec::with_capacity(v.len()));
    }
    for row in v {
        for (j, element) in row.into_iter().enumerate() {
            transposed[j].push(element);
        }
    }
    transposed
}

#[inline]
fn rotate_piece(p: Vec<Vec<u16>>) -> Vec<Vec<u16>> {
    let rotation_rng: u8 = thread_rng().gen_range(0..4);
    let mut rot_p: Vec<Vec<u16>> = p.clone();

    for i in 0..4 {
        for j in 0..4 {
            rot_p[i][j] = match rotation_rng {
                1 => p[j][3 - i],
                2 => p[3 - i][3 - j],
                3 => p[3 - j][i],
                _ => p[i][j],
            };
        }
    }
    rot_p
}

#[inline]
fn manipulate_piece(p: &[Vec<u16>], b: &[[u16; 7]; 7]) -> (Vec<Vec<u16>>, Vec<[usize; 2]>) {
    // randomly rotate piece 0, 90, 180, 270 degrees
    let mut piece = rotate_piece(p.to_owned());

    // randomly mirror piece 0 or 1 times
    let mirror_rng: u8 = thread_rng().gen_range(0..2);

    if mirror_rng == 1 {
        piece = mirror_piece(piece);
    }

    // remove empty rows in m_piece
    piece.retain(|row| row.iter().any(|&x| x != 0));

    // remove empty columns in m_piece
    let mut piece = trans_piece(piece);
    piece.retain(|row| row.iter().any(|&x| x != 0));

    // 2. randomly pick a location for the piece in the left most empty spaces, for columns,
    let mut pos = Vec::new();
    // record empty spaces
    for i in 0..7 - (piece.len() - 1) {
        for j in 0..7 - (piece[0].len() - 1) {
            if b[i][j] == 0 {
                pos.push([i, j]);
            }
        }
    }

    (piece, pos)
}

fn solve_puzzle() {
    // track puzzle attempts
    let mut puzzle_attempts = 0;
    let mut _best_blanks = 49;

    // initiate board
    let (starting_board, pieces) = initiate_board();

    let mut piece_vector_index = [0, 1, 2, 3, 4, 5, 6, 7];

    'start_puzzle: loop {
        // Check if the puzzle has been solved by another thread
        if SOLVED.load(Ordering::Relaxed) {
            break 'start_puzzle;
        }

        // log attempts at the puzzle
        puzzle_attempts += 1;

        // make a copy of the starting board
        let mut board = starting_board;

        // 1. randomly shuffle the pieces, create range and shuffle it
        piece_vector_index.shuffle(&mut thread_rng());

        // iterate over the shuffled piece_vector_index placing two pieces in at a time
        for (piece_index_1, piece_index_2) in piece_vector_index
            .iter()
            .step_by(2)
            .zip(piece_vector_index.iter().skip(1).step_by(2))
        {
            let (piece_1, pos_1): (Vec<Vec<u16>>, Vec<[usize; 2]>) =
                manipulate_piece(&pieces[*piece_index_1], &board);
            if pos_1.is_empty() {
                continue 'start_puzzle;
            }

            let (piece_2, pos_2): (Vec<Vec<u16>>, Vec<[usize; 2]>) =
                manipulate_piece(&pieces[*piece_index_2], &board);
            if pos_2.is_empty() {
                continue 'start_puzzle;
            }

            // 3. loop through the coordinates fitting both pieces, use the two coordinates which
            // gives the minimal perimeter of smallest number of blank spaces after fitting
            let mut perimeter = 4 * 49;
            let mut min_perimeter_board = board;
            let mut sum_blanks = 49;
            let mut fit_piece = false;
            for (ni, nj) in iproduct!(0..pos_1.len(), 0..pos_2.len()) {
                let pos_i = &pos_1[ni];
                let pos_j = &pos_2[nj];

                if pos_i[0] != pos_j[0] && pos_i[1] != pos_j[1] {
                    let mut trial_board = board;

                    for i in 0..piece_1.len() {
                        for j in 0..piece_1[0].len() {
                            trial_board[pos_i[0] + i][pos_i[1] + j] += piece_1[i][j];
                        }
                    }

                    for i in 0..piece_2.len() {
                        for j in 0..piece_2[0].len() {
                            trial_board[pos_j[0] + i][pos_j[1] + j] += piece_2[i][j];
                        }
                    }

                    if valid_board(trial_board) {
                        let new_sum_blank_spaces = sum_blank_spaces(trial_board);
                        if new_sum_blank_spaces <= sum_blanks {
                            // calculate the perimeter of the trial_board
                            let new_perimeter = calc_perimeter(trial_board);
                            if new_perimeter < perimeter {
                                // if the piece fits update the board, and go to next piece
                                min_perimeter_board = trial_board;
                                perimeter = new_perimeter;
                                sum_blanks = new_sum_blank_spaces;
                                fit_piece = true;
                            }
                        }
                    }
                }
            }
            if !fit_piece {
                // couldnt fit piece, start over
                continue 'start_puzzle;
            }

            board = min_perimeter_board;
        }
        // // calculate number of blank spaces
        let blanks: u16 = sum_blank_spaces(board);

        // keep tally of the best number of blank spaces
        if blanks <= _best_blanks {
            _best_blanks = blanks;
            let best_board = board;
            println!("\nlowest number of blank spaces: {:?}\n", _best_blanks);
            println!("{}", format!("best solve so far:").bold().green());
            board_image(best_board);
            println!("completed puzzle_attempts: {:?}", puzzle_attempts);
        }

        // if the board is solved, print the board and exit
        if blanks == 0 {
            println!("\n\n\n{}", format!("solved board:").bold().purple());
            board_image(board);
            println!("\n{}", format!("you bloody legend, YOU!").bold().purple());

            // Set the SOLVED flag to true if the puzzle is solved
            SOLVED.store(true, Ordering::Relaxed);
            break 'start_puzzle;
        }

        if puzzle_attempts % 1_000_000_000 == 0 {
            println!(
                "thread completed: {:?} M. puzzle_attempts",
                puzzle_attempts / 1_000_000
            );
            println!("\n\n\n{}", format!("reached limit").bold().purple());
            break 'start_puzzle;
        }
    }
}

// How many cores do you want?
const THREAD_COUNT: usize = 10;

// Create an AtomicBool initialized to false
static SOLVED: AtomicBool = AtomicBool::new(false);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..THREAD_COUNT {
        handles.push(thread::spawn(|| solve_puzzle()));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
