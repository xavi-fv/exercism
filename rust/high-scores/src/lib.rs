#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self{scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.into_iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = Vec::from(self.scores);
        v.sort();
        v.reverse();

        if self.scores.len() < 3 {
            v
        } else {
            let (first, _) = v.split_at(3);
            Vec::from(first)
        }

        // menb111's is much simpler:
        // let mut res_vec = self.scores.to_vec();
        // res_vec.sort_unstable_by(|a, b| b.cmp(a));
        // res_vec.truncate(3);
        // res_vec
    }
}
