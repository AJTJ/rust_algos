use std::{collections::HashMap, vec};

/*
https://docs.rs/ndarray/latest/ndarray/
- ensures that every row is the same length
- keeps everything in one memory region
- Instead of vec[y][x], ndarray does vec[y * width + x]


 */

fn main() {
    fn depth_first_immutable(
        land: &Vec<Vec<u8>>,
        row_i: usize,
        col_i: usize,
        lake_cells_counter: &mut HashMap<(usize, usize), bool>,
    ) {
        // mark the cell off as viewed
        lake_cells_counter.insert((row_i, col_i), true);
        // let current_row = &land[row_i];

        let neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (n_row, n_col) in &neighbors {
            let (new_row, new_col) = (row_i as isize + n_row, col_i as isize + n_col);
            if new_row >= 0
                && new_row < land.len() as isize
                && new_col >= 0
                // this presume a FULL 2D array, which it should be...
                && new_col < land[0].len() as isize
            {
                let (new_row, new_col) = (new_row as usize, new_col as usize);
                if land[new_row][new_col] == 0
                    && !lake_cells_counter.contains_key(&(new_row, new_col))
                {
                    depth_first_immutable(land, new_row, new_col, lake_cells_counter);
                }
            }
        }
    }

    fn find_lakes_immutable(
        land: &Vec<Vec<u8>>,
        lake_counter: &mut i32,
        lake_cells_counter: &mut HashMap<(usize, usize), bool>,
    ) {
        for (i, row) in land.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 0 && lake_cells_counter.insert((i, j), true).is_none() {
                    *lake_counter += 1;
                    depth_first_immutable(&land, i, j, lake_cells_counter);
                }
            }
        }
    }

    let mut test_case: Vec<Vec<u8>> = vec![vec![1, 0, 0], vec![0, 1, 1], vec![1, 1, 1]];

    // NOTE: It would be simpler to simply mutate the 2D array
    // MUUUUUUCH SIMPLER
    let mut lake_counter = 0;
    let mut lake_cells_counter: HashMap<(usize, usize), bool> = HashMap::new();

    find_lakes_immutable(&test_case, &mut lake_counter, &mut lake_cells_counter);

    println!("lakes counter: {lake_counter}");

    //
    //
    //
    //
    //
    //
    //

    fn depth_first_mutating(land: &mut Vec<Vec<u8>>, row_i: usize, col_i: usize) {
        // mutate the lake so that it is land and not recounted
        land[row_i][col_i] = 1;
        let neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (n_row, n_col) in &neighbors {
            let (new_row, new_col) = (row_i as isize + n_row, col_i as isize + n_col);
            if new_row >= 0
                && new_row < land.len() as isize
                && new_col >= 0
                // this presume a FULL 2D array, which it should be...
                && new_col < land[0].len() as isize
            {
                let (new_row, new_col) = (new_row as usize, new_col as usize);
                if land[new_row][new_col] == 0 {
                    depth_first_mutating(land, new_row, new_col);
                }
            }
        }
    }

    fn find_lakes_mutating(land: &mut Vec<Vec<u8>>, lake_counter: &mut i32) {
        for i in 0..land.len() {
            for j in 0..land[i].len() {
                if land[i][j] == 0 {
                    *lake_counter += 1;
                    depth_first_mutating(land, i, j);
                }
            }
        }
    }

    let mut lake_counter_mutating = 0;

    find_lakes_mutating(&mut test_case, &mut lake_counter_mutating);

    println!("mutating lakes counter: {lake_counter_mutating}");
}

// UNUSED CODE

// // North
// if row_i > 0 {
//     if land[row_i - 1][col_i] == 0 && !lake_cells_counter.contains_key(&(row_i - 1, col_i))
//     {
//         depth_first(&land, row_i - 1, col_i, lake_cells_counter);
//     }
// }

// // East
// if current_row.len() > col_i + 1 {
//     if land[row_i][col_i + 1] == 0 && !lake_cells_counter.contains_key(&(row_i, col_i + 1))
//     {
//         depth_first(&land, row_i, col_i + 1, lake_cells_counter);
//     }
// }

// // South
// if land.len() > row_i + 1 {
//     if land[row_i + 1][col_i] == 0 && !lake_cells_counter.contains_key(&(row_i + 1, col_i))
//     {
//         depth_first(&land, row_i + 1, col_i, lake_cells_counter);
//     }
// }

// // West
// if col_i > 0 {
//     if land[row_i][col_i - 1] == 0 && !lake_cells_counter.contains_key(&(row_i, col_i - 1))
//     {
//         depth_first(&land, row_i, col_i - 1, lake_cells_counter);
//     }
// }
