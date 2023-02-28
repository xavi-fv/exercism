use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut sorted_word: Vec<char> = word.chars().collect();
    sorted_word.sort_unstable();

    let mut anagrams = HashSet::<&'a str>::new();

    for &possible_anagram in possible_anagrams {
        let possible_anagram_lower = possible_anagram.to_lowercase();
        if possible_anagram_lower == word { continue; }

        let mut chars = possible_anagram_lower.chars().collect::<Vec<char>>();
        chars.sort_unstable();

        if chars == sorted_word {
            anagrams.insert(possible_anagram);
        }
    }
    anagrams
}
