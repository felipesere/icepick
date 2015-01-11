struct Configuration {
    choices: Vec<String>,
}

impl Configuration {
    fn from_inputs(choice: Vec<String>)  -> Configuration {
        let cleaned = choice.iter().map(clean as fn(&String) -> String).collect::<Vec<String>>();

        Configuration { choices: cleaned }
    }

    fn choices(self) -> Vec<String> {
        self.choices
    }

}

fn clean(input: &String) -> String {
    let slice = input.as_slice();
    slice.trim_left().trim_right().to_string()
}



#[cfg(test)]

#[test]
fn removes_leading_and_trailing_whitespace() {
    let mut input: Vec<String> = Vec::new();
    input.push(" a choice ".to_string());

    let config = Configuration::from_inputs(input);

    assert_eq!(config.choices(), vec!("a choice"));
}

#[test]
fn can_specify_initial_search() {

}
