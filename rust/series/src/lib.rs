struct WindowIterator<'a> {
    digits: Option<&'a str>,
    len: usize
}

impl<'a> WindowIterator<'a> {
    fn new(digits: &'a str, len: usize ) -> Self {
        Self { digits: Some(digits), len }
    }
}

impl<'a> Iterator for WindowIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len <= self.digits?.len() {
            let res = &self.digits?[..self.len];
            self.digits = self.digits?.get(1..);
            Some(res.to_string())
        } else {
            None
        }
    }
}

pub fn series(digits: &str, len: usize) -> Vec<String> {
    WindowIterator::new(digits, len).collect()
}
