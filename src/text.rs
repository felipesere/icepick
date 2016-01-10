use score::Match;

#[derive(PartialEq, Debug)]
pub enum Text<'a> {
    Colored(Match<'a>),
    Normal(String),
    Highlight(String),
    Blank,
}

impl<'a> Text<'a> {
    pub fn printable(self) -> String {
        match self {
            Text::Normal(t) => t,
            _ => panic!("only normal is plrintable?")
        }
    }
}
