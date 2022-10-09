#[derive(Debug)]
pub struct HighScores<'a> {
    all_scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { all_scores: scores }
    }

    pub fn scores(&self) -> &'a [u32] {
        self.all_scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.all_scores.to_vec().pop()
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.all_scores.into_iter().max() {
            Some(score) => Some(*score),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut vec = self.all_scores.to_vec();
        vec.sort_by(|a, b| b.cmp(a));
        // let mut v = Vec::<u32>::new();
        // while v.len() < 3 {
        //     match vec.pop() {
        //         Some(score) => v.push(score),
        //         None => return v,
        //     }
        // }
        // v
        vec.truncate(3);
        vec
    }
}
