fn main() {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut first_index: Option<i32> = None;
        let mut second_index: Option<i32> = None;
        let mut first_value: Option<i32> = None;
        let mut second_value: Option<i32> = None;

        for (i, x) in nums.iter().enumerate() {
            if first_value == None {
                first_value = Some(*x);
            }
            for (i, y) in nums.iter().enumerate() {
                if y + first_value.unwrap() == target {
                    second_index = Some(*y);
                    break;
                }
            }
        }

        let my_vec: Vec<i32> = vec![first_index.unwrap(), second_index.unwrap()];
        my_vec
    }

    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));

    println!("{:?}", alt_two_sum(vec![2, 7, 11, 15], 9));
}

// THIS IS PROBABLY THE GOOD ONE
pub fn alt_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut first_index: Option<usize> = None;
    let mut second_index: Option<usize> = None;

    'outer: for (i, x) in nums.iter().enumerate() {
        let current_value = Some(*x);
        first_index = Some(i);
        for (i, y) in nums.iter().enumerate().skip(i + 1) {
            if y + current_value.unwrap() == target {
                second_index = Some(i);
                break 'outer;
            }
        }
    }

    let my_vec: Vec<i32> = vec![first_index.unwrap() as i32, second_index.unwrap() as i32];
    my_vec
}

// fn two_sums_b(nums: &[i32], target: i32) -> Option<(usize, usize)> {
//     nums.iter()
//         .copied()
//         .enumerate()
//         .find_map(|(i, x)| {
//             nums.iter()
//                 .copied()
//                 .enumerate()
//                 .skip(i + 1)
//                 .find(|&(j, y)| x + y == target)
//                 .map(|(j, _y)| (i, j))
//         })
// }

// fn two_sums_b(nums: &[i32], target: i32) -> Option<(usize, usize)> {
//     for (i_x,x) in nums.iter().enumerate() {
//         for (i_y,y) in nums.iter().enumerate().skip(i_x + 1) {
//             if x + y == target {
//                 return Some((i_x, i_y));
//             }
//         }
//     }
//     None
// }

// fn main() {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut first_index: Option<usize> = None;
//         let mut second_index: Option<usize> = None;

//         'outer: for (i, x) in nums.iter().enumerate() {
//             // set the current value
//             let current_value = Some(*x);
//             // set the first index to the index of the value being checked
//             first_index = Some(i);
//             // internal loop to check if any other number plus the current_value equals the target value
//             println!("cur val: {:?}", current_value.unwrap());
//             for (i, y) in nums.iter().enumerate().filter(|&(ind, _)| ind > i) {
//                 println!(
//                     "{} + {} = {}",
//                     y,
//                     current_value.unwrap(),
//                     y + current_value.unwrap()
//                 );
//                 if y + current_value.unwrap() == target {
//                     println!("EQUALS! {}, {}", y, current_value.unwrap());
//                     second_index = Some(i);
//                     break 'outer;
//                 }
//             }
//         }

//         let my_vec: Vec<i32> = vec![first_index.unwrap() as i32, second_index.unwrap() as i32];
//         my_vec
//     }

//     println!("{:?}", two_sum(vec![2, 7, 11, 15], 9))
// }
