fn create_fibonacci(vec: Option<Vec<i32>>, steps_remaining: i32) -> Option<Vec<i32>> {
    match vec {
        None => {
            let v: Vec<i32> = vec![0];
            return create_fibonacci(Some(v.clone()), steps_remaining - 1);
        }
        Some(mut v) => {
            if v.len() == 1 {
                v.push(1);
                create_fibonacci(Some(v.clone()), steps_remaining - 1);
            };
            v.push(v[v.len() - 1] + v[v.len() - 2]);
            if steps_remaining > 2 {
                return create_fibonacci(Some(v.clone()), steps_remaining - 1);
            } else {
                Some(v)
            }
        }
    }
}

fn main() {
    let fib = create_fibonacci(None, 45);
    println!("Sequence: {:?}", fib);
}
