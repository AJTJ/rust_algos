use ndarray::Array2;

use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

pub fn breadth_first_2d_vec(land: Vec<Vec<u8>>) -> u32 {
    let mut cells_visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut lake_count = 0;

    for i in 0..land.len() {
        for j in 0..land[i].len() {
            // if is lake and NOT visited
            if land[i][j] == 0 && !cells_visited.contains_key(&(i, j)) {
                // inc the lake
                lake_count += 1;

                // set this new root as visited
                cells_visited.insert((i, j), true);

                // create and add to the queue
                let mut queue = VecDeque::new();
                queue.push_front((i, j));

                // loop through cue
                while let Some(cur) = queue.pop_front() {
                    let (x, y) = cur;

                    // get neighbours and bounds check them
                    let neighbours = [
                        (Some(x), y.checked_sub(1)),
                        (x.checked_sub(1), Some(y)),
                        (x.checked_add(1), Some(y)),
                        (Some(x), y.checked_add(1)),
                    ];

                    for (n_i, n_j) in neighbours {
                        if let (Some(n_i), Some(n_j)) = (n_i, n_j) {
                            let potential_n = land.get(n_i).and_then(|row| row.get(n_j));

                            if potential_n.is_some()
                                && potential_n.unwrap() == &0
                                && !cells_visited.contains_key(&(n_i, n_j))
                            {
                                // add to cue
                                cells_visited.insert((n_i, n_j), true);
                                queue.push_front((n_i, n_j));
                            }
                        }
                    }
                }
            }
        }
    }

    return lake_count;
}

/*
- we have 2d grid and start coords and end coords
- performing a BFS from start and end to achieve an efficient computational time
- we keep track of cells visited separately?
- keep track of the start parents and end parents to retrace the path
- we need to perform a simultaneous step for both the start and end BFS'
- And after each step, we need to check if the current cell has been visited yet, by either
- from that shared coordinates, we retrace a path
- print out a list of coordinates... or we could create a new vec with the path as 1s

*/

pub fn breadth_first_2d_ndarray(
    land: Array2<char>,
    start: (usize, usize),
    end: (usize, usize),
) -> u32 {
    // cells visited for start and end
    let mut cells_visited: HashMap<(usize, usize), bool> = HashMap::new();

    // parent hashmaps to retrace route
    let mut start_parents: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut end_parents: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // queues for start and end points
    let mut start_queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut end_queue: VecDeque<(usize, usize)> = VecDeque::new();

    // check if the top element of each LILO queue is in existance of any of the parent lists?
    while !start_queue.is_empty() || !end_queue.is_empty() {
        if let Some(x) = start_queue.front() {
            if end_parents.contains_key(x) {
                // return the routes
            }
        }

        if let Some(x) = end_queue.front() {
            if start_parents.contains_key(x) {
                // return the routes
            }
        }

        if let Some((x, y)) = start_queue.pop_front() {
            let neighbours = [
                (Some(x), y.checked_sub(1)),
                (x.checked_sub(1), Some(y)),
                (x.checked_add(1), Some(y)),
                (Some(x), y.checked_add(1)),
            ];

            for (n_i, n_j) in neighbours {
                if let (Some(n_i), Some(n_j)) = (n_i, n_j) {
                    // we have a new coordinate that is real (not off the grid)
                    // we add it to the parent hash
                    start_queue.push_front((n_i, n_j));
                    start_parents.insert((n_i, n_j), (x, y));
                }
            }
        }
    }

    // fn find_routes

    // for ((i, j), y) in land.indexed_iter() {
    //     if land[(i, j)] == 0 && !cells_visited.contains_key(&(i, j)) {
    //         // set this new root as visited
    //         cells_visited.insert((i, j), true);

    //         // create and add to the queue
    //         let mut queue = VecDeque::new();
    //         queue.push_front((i, j));

    //         // loop through queue
    //         while let Some(cur) = queue.pop_front() {
    //             let (x, y) = cur;

    //             // get neighbours and bounds check them
    //             let neighbours = [
    //                 (Some(x), y.checked_sub(1)),
    //                 (x.checked_sub(1), Some(y)),
    //                 (x.checked_add(1), Some(y)),
    //                 (Some(x), y.checked_add(1)),
    //             ];

    //             for (n_i, n_j) in neighbours {
    //                 if let (Some(n_i), Some(n_j)) = (n_i, n_j) {
    //                     // let potential_n = land.get(n_i).and_then(|row| row.get(n_j));
    //                     let potential_n = land.get((n_i, n_j));

    //                     if potential_n.is_some()
    //                         && potential_n.unwrap() == &0
    //                         && !cells_visited.contains_key(&(n_i, n_j))
    //                     {
    //                         // add to cue
    //                         cells_visited.insert((n_i, n_j), true);
    //                         queue.push_front((n_i, n_j));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // for i in 0..land.len() {
    //     for j in 0..land[i].len() {
    // if is lake and NOT visited
    // if land[i][j] == 0 && !cells_visited.contains_key(&(i, j)) {
    //     // inc the lake
    //     lake_count += 1;

    //     // set this new root as visited
    //     cells_visited.insert((i, j), true);

    //     // create and add to the queue
    //     let mut queue = VecDeque::new();
    //     queue.push_front((i, j));

    //     // loop through cue
    //     while let Some(cur) = queue.pop_front() {
    //         let (x, y) = cur;

    //         // get neighbours and bounds check them
    //         let neighbours = [
    //             (Some(x), y.checked_sub(1)),
    //             (x.checked_sub(1), Some(y)),
    //             (x.checked_add(1), Some(y)),
    //             (Some(x), y.checked_add(1)),
    //         ];

    //         for (n_i, n_j) in neighbours {
    //             if let (Some(n_i), Some(n_j)) = (n_i, n_j) {
    //                 let potential_n = land.get(n_i).and_then(|row| row.get(n_j));

    //                 if potential_n.is_some()
    //                     && potential_n.unwrap() == &0
    //                     && !cells_visited.contains_key(&(n_i, n_j))
    //                 {
    //                     // add to cue
    //                     cells_visited.insert((n_i, n_j), true);
    //                     queue.push_front((n_i, n_j));
    //                 }
    //             }
    //         }
    //     }
    // }
    // }
    // }

    return lake_count;
}
