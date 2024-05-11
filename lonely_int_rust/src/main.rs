fn print_lonely_int(int_vec: Vec<i32>) {
    // needed to take ownership or do iter().copied() to create a new
    // let reduced = int_vec.iter().reduce(|a, b| a ^ b);
    println!("{}", int_vec.into_iter().reduce(|a, b| a ^ b).unwrap());
}

fn main() {
    let int_vec: Vec<i32> = vec![7, 7, 1, 9, 2, 3, 2, 1, 9];
    print_lonely_int(int_vec);
}
