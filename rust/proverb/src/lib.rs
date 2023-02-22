pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return "".to_string()
    }

    let mut lines: Vec<String> = vec![];

    for (word0, word1) in list[0..list.len()].into_iter().zip(&list[1..]) {
        lines.push(format!("For want of a {} the {} was lost.", word0, word1));
    }
    lines.push(format!("And all for the want of a {}.", list.first().unwrap()));
    lines.join("\n")
}


// Super nice and elegant nickers' solution
// use std::iter::once;
// pub fn build_proverb(list: &[&str]) -> String {
//     match list.first() {
//         None => String::new(),
//         Some(word) => list.windows(2)
//             .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
//             .chain(once(format!("And all for the want of a {}.", word)))
//             .collect(),
//     }
// }
