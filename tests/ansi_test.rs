extern crate icepick;

#[cfg(test)]
mod tests {
    use icepick::ansi::Ansi;
    use icepick::fake_tty::FakeIO;

    pub fn assert_results_in<F: FnMut(&mut Ansi)>(expected: &str, mut f: F) {
        let mut ansi = Ansi {
            io: Box::new(FakeIO::new()),
        };

        f(&mut ansi);
        let inner_box = ansi.io;
        assert_eq!(inner_box.last(), expected);
    }

    #[test]
    fn escapes_a_str() {
        assert_results_in("\x1b[something", |ansi| ansi.escape("something"));
    }

    #[test]
    fn clears_the_screen() {
        assert_results_in("\x1b[2J", |ansi| ansi.clear());
    }

    #[test]
    fn hides_the_cursor() {
        assert_results_in("\x1b[?251", |ansi| ansi.hide_cursor());
    }

    #[test]
    fn shows_the_cursor() {
        assert_results_in("\x1b[?25h", |ansi| ansi.show_cursor());
    }

    #[test]
    fn sets_the_position() {
        assert_results_in("\x1b[9;13H", |ansi| {
            ansi.set_position(8, 12);
        });
    }

    #[test]
    fn prints_inverted() {
        assert_results_in("\x1b[7mtest\x1b[0m", |ansi| {
            ansi.inverted("test");
        });
    }
}
