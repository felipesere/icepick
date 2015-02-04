use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub struct ScoreResult<'a> {
    pub quality: f32,
    pub choice: &'a String,
}

impl<'a> Ord for ScoreResult<'a> {
    fn cmp(&self, other: &ScoreResult) -> Ordering {
        // Reverses ordering to make the binary max heap a min heap in Search::filter.
        self.quality.partial_cmp(&other.quality).unwrap_or(Ordering::Equal).reverse()
    }
}

impl<'a> PartialOrd for ScoreResult<'a> {
    fn partial_cmp(&self, other: &ScoreResult) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for ScoreResult<'a> {
}

impl<'a> PartialEq for ScoreResult<'a> {
    fn eq(&self, other: &ScoreResult) -> bool {
       self.quality == other.quality
    }
}

pub struct SortedResultSet<'s> {
    results: BinaryHeap<ScoreResult<'s>>,
    size: usize,
}

impl<'s> SortedResultSet<'s> {
    pub fn new(size: usize) -> SortedResultSet<'s> {
        SortedResultSet {
            results:  BinaryHeap::with_capacity(size + 1),
            size:     size}
    }
    
    pub fn push(&mut self, choice: &'s String, quality: f32) {
        let result = ScoreResult { quality: quality, choice: choice};

        if self.results.len() < self.size {
            self.results.push(result);
        } else {
            self.results.push_pop(result);
        }
    }

    pub fn sorted_vec(self) -> Vec<String> {
        self.results.into_sorted_vec().iter().map(|score_result| score_result.choice.clone()).collect()
    }
}
