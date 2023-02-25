pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_question = message.ends_with('?');
    let is_yelling = message.chars().into_iter()
        .any(|c| c.is_alphabetic()) && message == message.to_uppercase();

    match (message.is_empty(), is_question, is_yelling) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, false) => "Sure.",
        (_, false, true) => "Whoa, chill out!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        _ => "Whatever."
    }
}


// Alternatively, we can use the "switch pattern":
//  match message.trim() {
//         m if m.len() == 0 => "Fine. Be that way!",
//         m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
//         m if m.ends_with("?") => "Sure.",
//         m if is_yelling(m) => "Whoa, chill out!",
//         _ => "Whatever."
//     }
