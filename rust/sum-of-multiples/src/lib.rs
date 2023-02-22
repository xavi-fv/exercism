use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut numbers = HashSet::<u32>::new();

    factors.iter()
        .for_each(|factor|
            (1..limit).into_iter()
                .map(|n| n * factor)
                .filter(|n| n < &limit)
                .for_each(|n| { numbers.insert(n); }
                )
        );
    numbers.iter().sum()
}
