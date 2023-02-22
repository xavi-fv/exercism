fn is_prime(p: u64) -> bool {
    let end = (p as f32).sqrt() as u64;
    (2..=end).into_iter().all(|n| p % n != 0)
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut result: Vec<u64> = vec![];
    let mut prime= 2u64;

    while num > 1 {
        if num % prime == 0 {
            result.push(prime);
            num /= prime;
        } else {
            // Next prime
            loop  {
                prime += 1;
                if is_prime(prime) {
                    break;
                }
            }
        }
    }
    result
}
