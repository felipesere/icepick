use tty::IO;

pub struct FakeIO {
    last: String,
}

impl FakeIO {
    pub fn new() -> FakeIO {
        FakeIO {  last: "".to_string() }
    }

}

impl IO for FakeIO {
   fn write(&mut self, line: &str) {
       self.last = line.to_string();
   }

   fn read(&mut self) -> Option<char> {
       None
   }

   fn last(&self) -> &str {
       self.last.as_slice()
   }
}
