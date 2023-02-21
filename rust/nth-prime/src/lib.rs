fn is_prime(p: u32) -> bool {
    let end = (p as f32).sqrt() as u32;
    (2..=end).into_iter().all(|n| p % n != 0)
}

pub fn nth(n: u32) -> u32 {
    (2u32..).into_iter()
        .filter(|x| is_prime(*x))
        // .take((n + 1) as usize)
        // .last()
        // parhamrm's solution is smarter: he proposes using `nth` instead of `take` + `last`
        .nth(n as usize + 1)
        .unwrap()
}
