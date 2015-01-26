use tty::IO;

pub struct FakeIO {
    last: String,
    lines: Vec<String>,
}

impl FakeIO {
    pub fn new() -> FakeIO {
        FakeIO {
            last: "fail".to_string(),
            lines: Vec::new(),
        }
    }

}

impl IO for FakeIO {
   fn write(&mut self, line: &str) {
       self.last = line.to_string();
       self.lines.push(line.to_string());
   }

   fn read(&mut self) -> Option<String> {
       None
   }

   fn lines(&self) -> Vec<String> {
       self.lines.clone()
   }

   fn last(&self) -> &str {
       self.last.as_slice()
   }
}
