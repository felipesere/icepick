use score;
use score::Match;
use sorted_result_set::SortedResultSet;
use std::slice::SliceExt;
use std::cmp::min;
use std::ascii::AsciiExt;

#[derive(Debug)]
pub struct Search<'s> {
    pub query: String,
    pub current: usize,
    pub result: Vec<Match<'s>>,
    choice_stack: ChoiceStack<'s>,
    pub visible_limit: usize,
    done: bool,
}

#[derive(Debug)]
struct ChoiceStack<'s> {
    content: Vec<Vec<&'s String>>,
}

impl <'s>ChoiceStack<'s> {
    pub fn new(input: &'s Vec<String>) -> ChoiceStack<'s> {
        let initial_choices = input.iter().map(|x| x).collect();

        ChoiceStack { content: vec![initial_choices] }
    }

    pub fn push(&mut self, frame: Vec<&'s String>) {
        self.content.push(frame);
    }

    pub fn pop(&mut self) {
        if self.content.len() > 1 {
            self.content.pop();
        }
    }

    pub fn peek(&self) -> &Vec<&'s String> {
        self.content.last().unwrap()
    }

    pub fn last_size(&self) -> usize {
        self.peek().len()
    }
}

impl<'s> Search<'s> {
    pub fn blank(choices: &'s Vec<String>,
                 initial_search: Option<String>,
                 visible_limit: usize) -> Search<'s> {
        let query = initial_search.unwrap_or("".to_string());

        let choice_stack = ChoiceStack::new(&choices);

        let result = choices.iter().take(visible_limit).map(|x| Match::origin_only(x)).collect();

        Search::new(query, choice_stack, result, 0, visible_limit, false)
    }

    fn new(query: String, choice_stack: ChoiceStack<'s>, result: Vec<Match<'s>>, index: usize, visible_limit: usize, done: bool) -> Search<'s> {
        Search { current: index,
                 query: query,
                 result: result,
                 choice_stack: choice_stack,
                 visible_limit: visible_limit,
                 done: done}
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn done(self) -> Search<'s> {
        Search::new(self.query, self.choice_stack, self.result, self.current, self.visible_limit, true)
    }

    pub fn selection(&self) -> Option<String> {
        self.result.get(self.current).map( |t| t.original.clone())
    }

    fn new_for_index(self, index: usize) -> Search<'s> {
        Search::new(self.query, self.choice_stack, self.result, index,self.visible_limit, self.done)
    }

    pub fn iter_matches<F: FnMut(Match<'s>)>(query: &str, choices: &Vec<&'s String>, mut f: F) {
        let lower_query = query.to_ascii_lowercase();

        for choice in choices.iter() {
            match score::score(&choice, &lower_query) {
                None     => continue,
                Some(m) => f(m),
            };
        }
    }

    pub fn down(self) -> Search<'s> {
        let next_index = self.next_index();
        self.new_for_index(next_index)
    }

    pub fn up(self) -> Search<'s> {
        let next_index = self.prev_index();
        self.new_for_index(next_index)
    }

    pub fn append_to_search(mut self, input: &str) -> Search<'s> {
        let mut new_query = self.query.clone();
        new_query.push_str(input.as_slice());

        let mut heap = SortedResultSet::new(self.visible_limit);
        let mut filtered_choices: Vec<&String> = Vec::new();
        Search::iter_matches(new_query.as_slice(), &self.choice_stack.peek(),
                        |matching| {
                                               let quality = matching.quality.to_f32();
                                               let choice = matching.original;
                                               heap.push(matching.clone(), quality);
                                               filtered_choices.push(&choice)
                                             });

        self.choice_stack.push(filtered_choices);

        let result =  heap.as_sorted_vec();

        Search::new(new_query, self.choice_stack, result, 0, self.visible_limit, self.done)
    }

    pub fn backspace(mut self) -> Search<'s> {
        let mut new_query = self.query.clone();
        new_query.pop();

        self.choice_stack.pop();

        let mut heap = SortedResultSet::new(self.visible_limit);
        Search::iter_matches(new_query.as_slice(), &self.choice_stack.peek(),
                             |matching| {
                                 let quality = matching.quality.to_f32();
                                 heap.push(matching, quality)
                             } );

        let result =  heap.as_sorted_vec();

        Search::new(new_query, self.choice_stack, result, 0, self.visible_limit, self.done)
    }

    fn next_index(&self) -> usize {
        let next_index = self.current + 1;

        if next_index >= self.actual_limit() {
            0
        } else {
            next_index
        }
    }

    fn prev_index(&self) -> usize {
        if self.current == 0 {
            self.actual_limit() - 1
        } else {
            self.current - 1
        }
    }

    fn actual_limit(&self) -> usize {
        min(self.visible_limit, self.num_matches())
    }

    pub fn num_matches(&self) -> usize {
        self.choice_stack.last_size()
    }
}
