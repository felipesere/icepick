#[derive(PartialEq, Show)]
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


#[cfg(test)]

#[test]
pub fn normal_is_a_simple_string() {
    let normal = Text::Normal("test".to_string());
    assert_eq!("test", normal.printable().as_slice());
}

