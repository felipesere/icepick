#![feature(plugin)]
#![plugin(stainless)]
#![feature(core)]

extern crate stainless;
extern crate athena;

mod test {
    pub use athena::text::Text;

    describe! text_test {
        it "normal_is_a_simple_string" {
            let normal = Text::Normal("test".to_string());
            assert_eq!("test", normal.printable().as_slice());
        }
    }
}
