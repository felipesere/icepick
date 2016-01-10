use score::Match;
use ansi::Ansi;
use std::cmp::min;
use ansi_term::Colour::Blue;

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

    fn printable_length(&self, text: &str, max_width: usize) -> usize {
        min(text.len(), max_width)
    }
}

pub trait Printable {
    fn print(self, ansi: &mut Ansi, max_width: usize);
}

impl <'a> Printable for Text<'a> {
    fn print(self, ansi: &mut Ansi , max_width: usize) {
        match self {
            Text::Colored(ref matching) => {
                let (start, middle, end) = matching.parts();
                let text = format!("{}{}{}", start, Blue.paint(middle.as_ref()), end);
                let printable_length = self.printable_length(text.as_ref(), max_width);
                ansi.print(&text[..printable_length]);
            }
            Text::Normal(ref text) => {
                let printable_length = self.printable_length(text, max_width);
                ansi.print(&text[..printable_length]);
            }
            Text::Highlight(ref text) => {
                let printable_length = self.printable_length(text, max_width);
                ansi.inverted(&text[..printable_length]);
            }
            Text::Blank => ansi.print("".as_ref()),
        };
    }
}
