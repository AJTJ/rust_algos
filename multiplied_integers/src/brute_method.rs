const MAX_LIMIT: u32 = 100;

fn find_if_working(
    norm: (u32, u32, u32, u32),
    powed: (u32, u32, u32, u32),
    vec: &mut Vec<Vec<u32>>,
) -> (bool, &mut Vec<Vec<u32>>) {
    let (a, b, c, d) = norm;
    let (a_pow, b_pow, c_pow, d_pow) = powed;
    let mut algo_works: bool = false;
    if a_pow + b_pow == c_pow + d_pow {
        vec.push(vec![a, b, c, d]);
        algo_works = true;
    }
    (algo_works, vec)
}

fn loop_through() -> Vec<Vec<u32>> {
    let mut possible_combinations: Vec<Vec<u32>> = vec![];
    let mut a;
    let mut a_pow;
    let mut b;
    let mut b_pow;
    let mut c;
    let mut c_pow;
    let mut d;
    let mut d_pow;
    for a_int in 1..MAX_LIMIT {
        a = a_int as u32;
        a_pow = a_int.pow(3) as u32;
        for b_int in 1..MAX_LIMIT {
            b = b_int as u32;
            b_pow = b_int.pow(3) as u32;
            for c_int in 1..MAX_LIMIT {
                c = c_int as u32;
                c_pow = c_int.pow(3) as u32;
                for d_int in 1..MAX_LIMIT {
                    d = d_int as u32;
                    d_pow = d_int.pow(3) as u32;
                    let (algo_works, _) = find_if_working(
                        (a, b, c, d),
                        (a_pow, b_pow, c_pow, d_pow),
                        &mut possible_combinations,
                    );
                    // there is only one possible d for any combination of a, b and c
                    if algo_works {
                        break;
                    }
                }
            }
        }
    }

    possible_combinations
}

pub fn run_brute() {
    let result = loop_through();
    // println!("{:?}", result.len());
}
