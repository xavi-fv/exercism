pub fn is_leap_year(year: u64) -> bool {
    (year % 400 == 0) || ((year % 4 == 0) && (year % 100 > 0))

    // I also like a lot Eventide's solution:
    // (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}
