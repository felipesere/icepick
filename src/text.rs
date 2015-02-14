#[derive(PartialEq, Debug)]
pub enum Text {
    Normal(String),
    Highlight(String),
    Blank,
}

impl Text {
    pub fn printable(self) -> String {
        match self {
            Text::Normal(t) => t,
            _ => "fail".to_string(),
        }
    }
}
