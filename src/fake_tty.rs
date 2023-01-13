use crate::tty::IO;

pub struct FakeIO {
    last: String,
    lines: Vec<String>,
    input: Vec<String>,
}

impl FakeIO {
    pub fn new() -> FakeIO {
        FakeIO::new_with_input(vec![])
    }

    pub fn new_with_input(input: Vec<&str>) -> FakeIO {
        let actual = input.iter().map( |s| s.to_string()).collect();
        FakeIO {
            last: "fail".to_string(),
            lines: Vec::new(),
            input: actual,
        }
    }
}

impl IO for FakeIO {
   fn write(&mut self, line: &str) {
       self.last = line.to_string();
       self.lines.push(line.to_string());
   }

   fn read(&mut self) -> Option<String> {
        self.input.pop()
   }

   fn lines(&self) -> Vec<String> {
       self.lines.clone()
   }

   fn last(&self) -> &str {
       self.last.as_ref()
   }

   fn dimensions(&self) -> (usize, usize) {
       (50, 50)
   }

   fn reset(&self) {
   }
}
