#[derive(Copy, Clone)]
struct DigitsIterator {
    value: u32,
    end: bool
}

impl From<u32> for DigitsIterator {
    fn from(value: u32) -> Self {
        Self{value, end: false}
    }
}

impl Iterator for DigitsIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.value, self.end) {
            (0, false) => {
                self.end = true;
                Some(0)
            },
            (0, true) => None,
            (value, _) => {
                self.value /= 10;
                self.end = self.value == 0;
                Some(value % 10)
            }
        }
    }
}

pub fn is_armstrong_number(num: u32) -> bool {
    let iter = DigitsIterator::from(num);
    let num_digits = iter.into_iter().count() as u32;
    let sum = iter
        .into_iter()
        .map(|digit| digit.pow(num_digits))
        .fold(Some(0u32), |a, b| a?.checked_add(b));
    sum == Some(num)
}
