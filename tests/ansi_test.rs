#![feature(plugin)]
#![plugin(stainless)]

extern crate stainless;
extern crate athena;

mod test {
    use athena::fake_tty::FakeIO;
    use athena::tty::IO;
    use athena::ansi::Ansi;

    pub fn assert_results_in<F: FnMut(&mut Ansi)> (expected: &str, mut f: F) {
        let mut ansi = Ansi { io: Box::new(FakeIO::new()) };

        f(&mut ansi);
        let inner_box = ansi.io;
        assert_eq!(inner_box.last(), expected);
    }

    describe! ansi_test {
        it "escapes_a_str" {
            assert_results_in("\x1b[something", |ansi| { ansi.escape("something") });
        }

        it "clears_the_screen" {
            assert_results_in("\x1b[2J", |ansi| { ansi.clear() });
        }

        it "hides_the_cursor" {
            assert_results_in("\x1b[?251", |ansi| { ansi.hide_cursor() });
        }

        it "shows_the_cursor" {
            assert_results_in("\x1b[?25h", |ansi| { ansi.show_cursor() });
        }

        it "sets_the_position" {
            assert_results_in("\x1b[9;13H", |ansi| { ansi.set_position(8,12); });
        }

        it "prints_inverted" {
            assert_results_in("\x1b[7mtest\x1b[0m", |ansi| { ansi.inverted("test"); });
        }
    }
}
