use std::collections::HashMap;

fn word_map<'a>(text: &[&'a str]) -> HashMap::<&'a str, u32> {
    let mut map = HashMap::<&str, u32>::new();
    for word in text {
        let entry = map.entry(word).or_insert(0);
        *entry += 1;
    }
    map
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_map = word_map(magazine);
    let article_map = word_map(note);

    article_map.into_iter().all(|entry|
        if let Some(occurrences) = magazine_map.get(entry.0) {
            *occurrences >= entry.1
        } else { false })
}
