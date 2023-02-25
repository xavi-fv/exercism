pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut steps = 0u64;

    loop {
        if n <= 1 { break; }

        if n % 2 == 0 {
            n /= 2;
        } else if let Some(v1) = n.checked_mul(3) {
            if let Some(v2) = v1.checked_add(1) {
                n = v2;
            } else { return
                None;
            }
        } else { return None; }
        steps += 1;
    }
    if n == 1 { Some(steps) } else { None }
}


// Very nice jcsr's solution
pub fn collatz2(mut n: u64) -> Option<u64> {
    for i in 0.. {
        match n {
            0 => break,
            1 => return Some(i),
            even if even % 2 == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
    }
    None
}