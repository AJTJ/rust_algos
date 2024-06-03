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
                                cells_visited.insert((n_i, n_j), true);
                                // add to cue
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

pub fn breadth_first_2d_ndarray(
    land: Array2<char>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    // parent hashmaps to retrace route
    let mut start_parents: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    start_parents.insert(start, None);
    let mut end_parents: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    end_parents.insert(end, None);

    // queues for start and end points
    let mut start_queue: VecDeque<(usize, usize)> = VecDeque::new();
    start_queue.push_front(start);
    let mut end_queue: VecDeque<(usize, usize)> = VecDeque::new();
    end_queue.push_front(end);

    // flag if the paths are connected
    let mut connection: Option<(usize, usize)> = None;

    // TODO: Add the start and end points to their queues

    // check if the top element of each LILO queue is in existance of any of the parent lists?
    while !start_queue.is_empty() || !end_queue.is_empty() {
        println!("start_queue: {:?}", start_queue);
        if let Some(x) = start_queue.front() {
            if end_parents.contains_key(x) {
                connection = Some(*x);
                println!("break: {x:?}");
                break;
            }
        }
        investigate_step(&mut start_queue, &mut start_parents, &land);

        println!("end_queue: {:?}", end_queue);
        if let Some(x) = end_queue.front() {
            if start_parents.contains_key(x) {
                connection = Some(*x);
                println!("break: {x:?}");
                break;
            }
        }
        investigate_step(&mut end_queue, &mut end_parents, &land);

        fn investigate_step(
            my_queue: &mut VecDeque<(usize, usize)>,
            my_parents: &mut HashMap<(usize, usize), Option<(usize, usize)>>,
            land: &Array2<char>,
        ) {
            if let Some((x, y)) = my_queue.pop_front() {
                println!("looking at: {x}, {y}");
                let neighbours = [
                    (Some(x), y.checked_sub(1)),
                    (x.checked_sub(1), Some(y)),
                    (x.checked_add(1), Some(y)),
                    (Some(x), y.checked_add(1)),
                ];

                for (n_i, n_j) in neighbours {
                    if let (Some(n_i), Some(n_j)) = (n_i, n_j) {
                        // This is interesting. I don't actually need the grid to figure this out. This is presuming a logical grid with non-empty squares.

                        let potential_n = land.get((n_i, n_j));

                        if potential_n.is_some() && !my_parents.contains_key(&(n_i, n_j)) {
                            // we have a new coordinate that is real (not off the grid)
                            // we add it to the parent hash
                            my_queue.push_back((n_i, n_j));
                            my_parents.insert((n_i, n_j), Some((x, y)));
                        }
                    }
                }
            }
        }
    }

    if let Some(x) = connection {
        let mut current = start_parents.get(&x);
        let mut start_list: Vec<(usize, usize)> = vec![x];
        while let Some(el) = current.and_then(|inner| inner.as_ref()) {
            // println!("current: {:?}", current);
            start_list.push(*el);
            current = start_parents.get(el);
        }

        let mut current = end_parents.get(&x);
        let mut end_list: Vec<(usize, usize)> = vec![];
        while let Some(el) = current.and_then(|inner| inner.as_ref()) {
            end_list.push(*el);
            current = end_parents.get(el);
        }

        start_list.reverse();
        start_list.extend(&end_list);
        Some(start_list)
    } else {
        None
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
