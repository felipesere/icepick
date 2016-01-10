use score::Match;
use ansi::Ansi;
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
}

pub trait Printable {
    fn print(self, ansi: &mut Ansi);
}

impl <'a> Printable for Text<'a> {
    fn print(self, ansi: &mut Ansi) {
        match self {
            Text::Colored(ref matching) => {
                let (start, middle, end) = matching.parts();
                let text = format!("{}{}{}", start, Blue.paint(middle.as_ref()), end);
                ansi.print(&text);
            }
            Text::Normal(ref text) => {
                ansi.print(&text);
            }
            Text::Highlight(ref text) => {
                ansi.inverted(&text);
            }
            Text::Blank => ansi.print(""),
        };
    }
}
