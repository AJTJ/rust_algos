fn main() {
    let xor_list = vec![3, 4, 5, 6, 7, 8];
    let xor_totals = get_all_xors(xor_list);

    fn get_all_xors(xor_list: Vec<i32>) -> Vec<i32> {
        let mut current_xors: Vec<i32> = vec![];
        gen_xor(&mut vec![], xor_list, &mut current_xors);

        fn gen_xor(passed_vals: &mut Vec<i32>, xor_list: Vec<i32>, current_xors: &mut Vec<i32>) {
            for (i, el) in xor_list.clone().iter().enumerate() {
                let mut current_vals = passed_vals.clone();
                current_vals.push(*el);
                let current_xor_result = current_vals.iter().fold(0, |acc, x| acc ^ x);
                current_xors.push(current_xor_result);
                gen_xor(&mut current_vals, xor_list[i + 1..].to_vec(), current_xors);
            }
        }
        current_xors
    }

    let result = xor_totals.iter().fold(0, |acc, x| acc + x);
    println!("all xors: {:?}", result);
}
