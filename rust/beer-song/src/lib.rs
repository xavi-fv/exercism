trait Bottles {
    fn bottles(&self) -> String;
    fn bottles_of_beer(&self) -> String;
    fn bottles_of_beer_on_the_wall(&self) -> String;
    fn first_action(&self) -> String;
    fn second_action(&self) -> String;
}

impl Bottles for u32 {
    fn bottles(&self) -> String {
        match *self {
            0 => "No more bottles".to_string(),
            1 => "1 bottle".to_string(),
            _ => format!("{} bottles", *self),
        }
    }

    fn bottles_of_beer(&self) -> String {
        format!("{} of beer", self.bottles())
    }

    fn bottles_of_beer_on_the_wall(&self) -> String {
        format!("{} on the wall", self.bottles_of_beer())
    }

    fn first_action(&self) -> String {
        match *self {
            0 => "Go to the store",
            1 => "Take it down",
            _ => "Take one down",
        }
        .to_string()
    }

    fn second_action(&self) -> String {
        match *self {
            0 => "buy some more",
            _ => "pass it around",
        }
        .to_string()
    }
}

pub fn verse(n: u32) -> String {
    format!(
        "{}, {}.\n{} and {}, {}.\n",
        n.bottles_of_beer_on_the_wall(),
        n.bottles_of_beer().to_lowercase(),
        n.first_action(),
        n.second_action(),
        ((n + 99) % 100)
            .bottles_of_beer_on_the_wall()
            .to_lowercase()
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .into_iter()
        .map(verse)
        // Inferring type is much better than .collect::<Vec<String>>()
        .collect::<Vec<_>>()
        .join("\n")
}
