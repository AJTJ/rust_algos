// How is time complexity measured?
// By counting the number of primitive operations performed by the algorithm on a given input size

// constant complexity
// O(1) , i.e. the pidgeon
// always takes thes same amount of time
fn _return_double_index(i: usize, vec: Vec<i32>) -> i32 {
    vec[i * 2]
}

// logarithmic complexity
// O(logn) - Binary search
// time complexity = log(data input)
fn bin_search_for_index(
    arr: [i32; 8],
    starting_index: i32,
    end_index: i32,
    sought_num: i32,
) -> Option<f32> {
    // find the middle of the array
    if end_index >= starting_index {
        // find mid index
        let mid_index = starting_index as f32 + (((end_index - starting_index) / 2) as f32).floor();

        // check if this middle value is equal to the target
        let mid_value = arr[mid_index as usize];
        if mid_value == sought_num {
            return Some(mid_index);
        }

        // check if it is less than mid value
        if mid_value > sought_num {
            return bin_search_for_index(arr, starting_index, mid_index as i32 - 1, sought_num);
        }

        // check if it is greater than mid value
        return bin_search_for_index(arr, mid_index as i32 + 1, end_index, sought_num);
    } else {
        None
    }
}

// linear complexity
// O(n) time complexity = input size
fn _is_prime_all(num: i64) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

// loglinear complexity
// O(nlogn)
// basically means that "logn" operations occur "n" times
// common in recursive sorting algorithms, sorting algorithms using a binary tree, and many other types of sorts
// time compex = data input * log(data input)
fn quick_sort(all_nums: &mut Vec<i32>) -> Vec<i32> {
    let mut left = vec![];
    let mut right = vec![];
    let pivot = all_nums.pop();

    match pivot {
        Some(popped) => {
            for (i, x) in all_nums.iter().enumerate() {
                if all_nums[i] <= popped {
                    left.push(x.clone())
                } else {
                    right.push(x.clone())
                }
            }
            let mut return_vec = vec![];
            return_vec.extend(quick_sort(&mut left));
            return_vec.push(popped);
            return_vec.extend(quick_sort(&mut right));
            return_vec
        }
        None => {
            let mut return_vec = vec![];
            return_vec.extend(all_nums.clone());
            return_vec
        }
    }
}

// Quadratic time complexity
// O(n^2) or O(n*n) time complexity = input size * input size
fn _print_pairs(nums: Vec<u32>) {
    for x in &nums {
        for y in &nums {
            println!("{}, {}", x, y);
        }
    }
}

// Cubic time complexity
// O(n^3)

// Exponential time complexity
// O(x^n)
// extraordinarily rare, and bad
// tower of hanoi runs in O(2^n)

// Factorial time complexity
// O(n!)
// absolutely the slowest
// NP-complete problems: a type of problem where there is no known correct and fast solution

// HOW TO FIND TIME COMPLEXITY
// First rule: add up steps
// O(a+b)
fn _many_steps(do_step_1: fn() -> (), do_step_2: fn() -> ()) {
    do_step_1();
    do_step_2();
}

// Second rule: drop constants
// OPTION A and OPTION B are both O(n) NOT O(2n)
fn _min_max(vec: Vec<i32>) {
    // OPTION A
    for _ in &vec {
        // find min
    }
    for _ in &vec {
        // find max
    }

    // OPTION B
    for _ in &vec {
        // find min
        // and find max
    }
}

//third rule: different inputs = different variables
// O(a*b)
fn _find_intersection_count(a: Vec<u32>, b: Vec<u32>) {
    let mut count = 0;
    for a_int in &a {
        for b_int in &b {
            if a_int == b_int {
                count += 1
            }
        }
    }
    println!("{}", count);
}

// fourth rule: drop non-dominant terms
// this is O(n^2) NOT O(n + n^2)
fn _why_do(ve: Vec<u32>) {
    // O(n)
    for a in &ve {
        println!("{} do something", a)
    }

    // O(n^2)
    for a in &ve {
        for b in &ve {
            println!("{},{}", a, b);
        }
    }
}

// another O(logn)
fn is_log_n(n: i32) {
    let mut a = 0;
    let mut i = n;

    while i > 0 {
        println!("{}, {}", a, i);
        a = a + i;
        i /= 2;
    }
}

// O(logkn)
// for (let i = 0; i < n; i++) {
//     i *= k;
//   }

fn main() {
    let arr = [2, 3, 4, 10, 40, 50, 60, 70];
    let sought_num = 60;

    println!(
        "FINAL val index: {}",
        bin_search_for_index(arr, 0, arr.len() as i32 - 1, sought_num).unwrap()
    );

    let mut new_vec = vec![10, 2, 3, 7, 100, 55, 22, 88, 44];

    let sorted = quick_sort(&mut new_vec);
    println!("{:?}", sorted);

    // println!("{}", is_prime_all(11));
    // print_pairs(vec![1, 2, 3, 4, 5, 6, 7]);

    is_log_n(30);
}
