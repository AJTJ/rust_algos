fn fib(steps: u32) -> Vec<i32> {
    let mut cur_steps = steps - 2;
    let mut v: Vec<i32> = vec![0, 1];
    while cur_steps > 0 {
        v.push(v[v.len() - 1] + v[v.len() - 2]);
        cur_steps = cur_steps - 1
    }
    v
}

fn main() {
    println!("{:?}", fib(20))
}
