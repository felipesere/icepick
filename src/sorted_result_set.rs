use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub struct ScoreResult {
    pub quality: f32,
    pub choice: String,
}

impl Ord for ScoreResult {
    fn cmp(&self, other: &ScoreResult) -> Ordering {
        // Reverses ordering to make the binary max heap a min heap in Search::filter.
        self.quality.partial_cmp(&other.quality).unwrap_or(Ordering::Equal).reverse()
    }
}

impl PartialOrd for ScoreResult {
    fn partial_cmp(&self, other: &ScoreResult) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ScoreResult {
}

impl PartialEq for ScoreResult {
    fn eq(&self, other: &ScoreResult) -> bool {
       self.quality == other.quality
    }
}

pub struct SortedResultSet {
    results: BinaryHeap<ScoreResult>,
    size: usize,
}

impl SortedResultSet {
    pub fn new(size: usize) -> SortedResultSet {
        SortedResultSet {
            results:  BinaryHeap::with_capacity(size + 1),
            size:     size}
    }
    
    pub fn push(&mut self, quality: f32, choice: String) {
        let result = ScoreResult { quality: quality, choice: choice};

        if self.results.len() < self.size {
            self.results.push(result);
        } else {
            self.results.push_pop(result);
        }
    }

    pub fn sorted_vec(self) -> Vec<String> {
        self.results.into_sorted_vec().iter().map(|score_result| score_result.choice.to_string() ).collect::<Vec<String>>()
    }
}
