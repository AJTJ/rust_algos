fn main() {
    println!("is prime: {}", is_prime(12));
}

pub fn is_prime(n: i32) -> bool {
    for x in 2..n {
        if x * x > n {
            break;
        }

        if n % x == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::is_prime;

    #[test]
    fn it_works() {
        let test_values = vec![1, 5, 6, 7, 11, 15, 33, 37];

        let result: Vec<bool> = test_values.iter().map(|x| is_prime(*x)).collect();

        let correct_result = vec![true, true, false, true, true, false, false, true];

        assert_eq!(result, correct_result);
    }
}
