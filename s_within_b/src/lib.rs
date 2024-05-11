pub fn find_permutations() {
    // find all permutations of string s in string b
    let s = "xacxzaa";
    let b = "fxaazxacaaxzoecazxaxaz";

    // brute
    // first find all permutations of string s
    // loop over all permutations to see if they exist in string b
    // check each character in order of each permutation against the list of string b characters
    // this could be O(b) and it could be O(s^2b)

    let mut cases: u32 = 0;
    loop {
        let mut is_done = false;

        if is_done {
            break;
        }
    }

    // divide the s string into characters
    // find every situation that all characters are satisfied in whatever order

    let found = b.find(s).unwrap();
    println!("{}", found);
}
