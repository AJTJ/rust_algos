fn main() {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1, -1];
        fn bin_search(ve: Vec<i32>, target: i32, result: &mut Vec<i32>) {
            let start = 0 as usize;
            let end = ve.len() - 1;
            while start <= end {
                let mid = (start + end / 2) as i32;
                println!("{}, {}, {}", start, end, mid);
                if ve[mid as usize] == target {
                    // LEFT SIDE
                    if mid - 1 < 0 || ve[mid as usize - 1] != target {
                        if result[0] == -1 || result[0] > mid {
                            result[0] = mid;
                        }
                    }
                    if mid > 0 && ve[mid as usize - 1] == target {
                        bin_search(ve[0..(mid as usize - 1)].to_vec(), target, result);
                    }
                    // RIGHT SIDE
                    if mid + 1 >= ve.len() as i32 || ve[mid as usize + 1] != target {
                        if result[1] < mid {
                            result[1] = mid;
                        }
                    }
                    if (mid as usize) < ve.len() - 1 && ve[mid as usize + 1] == target {
                        bin_search(ve[(mid as usize + 1)..end].to_vec(), target, result);
                    }
                }
                // MAIN BIN SEARCH
                // left
                if ve[mid as usize] > target {
                    bin_search(ve[start..mid as usize - 1].to_vec(), target, result);
                }
                //right
                if ve[mid as usize] < target {
                    bin_search(ve[mid as usize + 1..end].to_vec(), target, result);
                }
            }
        }
        bin_search(nums, target, &mut result);
        result
    }

    search_range(vec![5, 7, 7, 8, 8, 10], 8);
}
