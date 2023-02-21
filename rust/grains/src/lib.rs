pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    //(2 as u64).pow(s - 1)
    1 << s - 1
}

pub fn total() -> u64 {
    // Sum of 2^i from i = 0 to N-1 = 2^N - 1, so the last unsigned number represented with
    // 64 bit is 2^64-1
    u64::MAX
}
