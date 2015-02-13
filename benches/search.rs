#![feature(test)]
#![feature(core)]

extern crate test;
extern crate selecta;
use self::test::Bencher;
use selecta::search::Search;
use selecta::sorted_result_set::SortedResultSet;

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
                                |choice, quality| results.push(choice, quality));
        results
    });
}
