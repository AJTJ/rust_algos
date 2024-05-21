use ndarray::Array2;

use std::collections::{HashMap, VecDeque};

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

pub fn breadth_first_2d_ndarray(land: Array2<u8>) -> u32 {
    let mut cells_visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut lake_count = 0;

    for ((i, j), y) in land.indexed_iter() {
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
