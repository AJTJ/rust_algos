use lakes_problem_queue_breadth_first::breadth_first_2d_ndarray;
use ndarray::Array2;
fn main() {
    println!("Hello, world!");

    let grid = Array2::from_shape_vec(
        (5, 7),
        vec![
            'S', 'O', 'O', 'X', 'O', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'O', 'O', 'X', 'O',
            'O', 'O', 'X', 'O', 'O', 'O', 'X', 'X', 'O', 'O', 'O', 'O', 'X', 'X', 'O', 'O', 'X',
            'G',
        ],
    )
    .unwrap();

    breadth_first_2d_ndarray(grid);
}
