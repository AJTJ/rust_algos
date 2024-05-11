use std::collections::HashMap;

pub mod brute_method;

// find all solutions of a^3 + b^3 = c^3 + d^3
// where 0 < a,b,c,d < 100
// ex: all same numbers is correct
// ex: they mirror each other

// brute force:
// where we loop through every possible integer. O(n^4)

// optimization
// a + b = c + d

const MAX_LIMIT: u32 = 100;

// a fn that loops over all possible integers and checks for successful equations
// this is looping through the range of integers
// with 3 internal loops
// Less brute force with hashmap
fn loop_through(c_d_hash: HashMap<u32, Vec<(u32, u32)>>) -> Vec<Vec<u32>> {
    let mut possible_combinations: Vec<Vec<u32>> = vec![];
    let mut a;
    let mut b;

    for a_int in 1..MAX_LIMIT {
        a = a_int as u32;
        for b_int in 1..MAX_LIMIT {
            b = b_int as u32;
            let a_b_sum = a.pow(3) + b.pow(3);
            match c_d_hash.get(&a_b_sum) {
                // there is a c_d value
                Some(c_d_vals) => {
                    for (c, d) in c_d_vals {
                        possible_combinations.push(vec![a, b, *c, *d])
                    }
                }
                None => break,
            }
        }
    }

    possible_combinations
}

// creating a hashmap of c^3 + d^3  (sum, Vec<(c, d))
// for this we would loop through c and d once
fn create_hash_table() -> HashMap<u32, Vec<(u32, u32)>> {
    let mut c_d_hash: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for c in 1..MAX_LIMIT {
        for d in 1..MAX_LIMIT {
            let sum: u32 = (c as u32).pow(3) + (d as u32).pow(3);
            match c_d_hash.get_mut(&sum) {
                Some(val) => {
                    // add to hash val
                    val.push((c, d));
                }
                None => {
                    // insert new hash
                    c_d_hash.insert(sum, vec![(c, d)]);
                }
            }
        }
    }
    c_d_hash
}

pub fn run_all() {
    let c_d_hash = create_hash_table();
    let result = loop_through(c_d_hash);
    // println!("{:?}", result.len());
}
