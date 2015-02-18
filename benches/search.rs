#![feature(test)]

extern crate test;
extern crate icepick;
use self::test::Bencher;
use icepick::search::Search;
use icepick::sorted_result_set::SortedResultSet;

fn one_two_three() -> Vec<String> {
    vec!["one".to_string(),
         "two".to_string(),
         "three".to_string()]
}

fn input_times(n: usize) ->Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for thing in one_two_three().iter().cycle().take(n) {
        result.push(thing.clone());
    }
    result
}

#[bench]
fn filter_speed(b: &mut Bencher) {
    let initial_elements = input_times(1000);
    let query = "t";
    let mut f = Vec::new();
    for g in initial_elements.iter() {
        f.push(g);
    }


    b.iter(||{
        let mut results = SortedResultSet::new(20);
        Search::iter_matches(query, &f,
                                |matching| {
                                    let quality = matching.quality.to_f32();
                                    results.push(matching, quality)
                                });
        results
    });
}
