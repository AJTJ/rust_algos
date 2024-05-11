use std::io;
fn main() {
    let user_value: i32 = loop {
        println!("Give a maximum number to find prime numbers within.");
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Response Error");

        println!("the response {}", response);

        match response.trim().parse::<i32>() {
            Ok(val) => {
                break val;
            }
            Err(_) => println!("That is not a valid number, try again."),
        };
    };

    print!("num is {}", user_value);

    let mut stored_primes = vec![];
    fn check_if_prime(x: i32, primes: &mut Vec<i32>) -> bool {
        if x % 2 == 0 {
            return false;
        }

        for p in primes.iter() {
            if *p != 1 && x % p == 0 {
                return false;
            }
        }

        primes.push(x);
        true
    }

    let nums = 1..user_value;
    nums.for_each(|x| {
        println!(
            "is {} a prime number? {}",
            x,
            check_if_prime(x, &mut stored_primes)
        )
    });
    println!("final list of primes: {:?}", stored_primes);
}
