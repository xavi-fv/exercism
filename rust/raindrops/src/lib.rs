pub fn raindrops(n: u32) -> String {
    let factors = [3u32, 5, 7];
    let sounds = ["Pling", "Plang", "Plong"];

    let sounds: String = factors
        .into_iter()
        .zip(sounds)
        .filter(|(factor, _)| n % *factor == 0)
        .map(|(_, sound)| sound)
        .collect();
    if sounds.is_empty() { n.to_string() } else { sounds }
}
