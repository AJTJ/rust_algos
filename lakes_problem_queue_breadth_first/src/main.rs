use lakes_problem_queue_breadth_first::breadth_first_2d_vec;

fn main() {
    let test_case: Vec<Vec<u8>> = vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1], vec![1, 1, 0]];

    println!("lakes: {}", breadth_first_2d_vec(test_case));
}
