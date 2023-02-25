pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '['|'('|'{' => { open.push(c); },
            ']'|')'|'}' => {
                let last_opened = open.pop();
                match last_opened {
                    Some('(') => { if c != ')' { return false; } },
                    Some('[') => { if c != ']' { return false; } },
                    Some('{') => { if c != '}' { return false; } },
                    None => { return false; }
                    _ => unreachable!(),
                }
            },
            _ => {}
        }
    }
    open.is_empty()
}
