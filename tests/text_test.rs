extern crate icepick;

#[cfg(test)]
mod tests {
    pub use icepick::text::Text;

    #[test]
    fn normal_is_a_simple_string() {
        let normal = Text::Normal("test".to_string());
        assert_eq!("test", normal.printable());
    }
}
